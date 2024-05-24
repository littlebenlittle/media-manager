package main

import (
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"github.com/fsnotify/fsnotify"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

var library map[string]gin.H = make(map[string]gin.H)

type Media struct {
	Title     string
	Format    string
	Shortname string
}

func main() {
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		log.Fatalf("%s", err)
	}
	watch("/mkv", watcher)
	watch("/webm", watcher)
	defer watcher.Close()

	clients := make(map[chan gin.H]struct{})
	go func() {
		for e := range watcher.Events {
			if e.Has(fsnotify.Create) {
				log.Printf("new media %s", e.Name)
				media := add_media(e.Name)
				for client := range clients {
					client <- media
				}
			}
		}
	}()
	go func() {
		for err := range watcher.Errors {
			log.Println("error:", err)
		}
	}()

	router := gin.Default()
	router.GET("/convert", convert)

	media_library := router.Group("/media_library")
	media_library.GET("", func(c *gin.Context) {
		log.Printf("sending %d items to client", len(library))
		c.JSON(http.StatusOK, library)
	})
	media_library.PUT("/:id", func(c *gin.Context) {
		id := c.Param("id")
		media, ok := library[id]
		if !ok {
			log.Printf("media not found: %s", id)
			c.AbortWithStatus(http.StatusNotFound)
			return
		}
		var update Media
		if err := c.BindJSON(&update); err != nil {
			log.Printf("json decode error: %s", err)
			c.AbortWithStatus(http.StatusBadRequest)
			return
		}
		media["title"] = update.Title
		media["format"] = update.Format
		media["shortname"] = update.Shortname
		c.Status(http.StatusAccepted)
	})
	media_library.GET("/events", func(c *gin.Context) {
		c.Writer.Header().Set("Content-Type", "text/event-stream")
		c.Writer.Header().Set("Cache-Control", "no-cache")
		c.Writer.Header().Set("Connection", "keep-alive")
		c.Writer.Header().Set("Transfer-Encoding", "chunked")
		c.Writer.Header().Set("X-Accel-Buffering", "no")
		log.Printf("new client %s", c.Request.RemoteAddr)
		client := make(chan gin.H)
		clients[client] = struct{}{}
		c.Stream(func(_ io.Writer) bool {
			t := make(chan struct{})
			go func() {
				time.Sleep(60 * time.Second)
				t <- struct{}{}
			}()
			select {
			case media, ok := <-client:
				if ok {
					log.Printf("sending event to client")
					c.SSEvent("new", media)
				}
				return ok
			case <-t:
				c.SSEvent("keepalive", nil)
				return true
			}
		})
		delete(clients, client)
		close(client)
	})

	log.Fatal(router.Run())
}

func watch(dir string, watcher *fsnotify.Watcher) {
	log.Printf("watching dir: %s", dir)
	if err := watcher.Add(dir); err != nil {
		log.Fatal(err)
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		log.Fatalf("%s", err)
	}
	for _, entry := range entries {
		log.Printf("entry: %s", entry)
		if !entry.Type().IsRegular() {
			log.Printf("not a regular file: %s", entry.Name())
			continue
		}
		add_media(filepath.Join(dir, entry.Name()))
	}
}

func add_media(path string) gin.H {
	id := uuid.New().String()
	title := filepath.Base(path)
	format := ""
	ext := filepath.Ext(path)
	if ext == "" {
		format = "unknown"
	} else {
		format = ext[1:]
	}
	media := gin.H{
		"id":        id,
		"title":     title,
		"path":      path,
		"format":    format,
		"shortname": title,
	}
	library[id] = media
	log.Printf("library has %d items", len(library))
	return media
}

func convert(c *gin.Context) {
	format := c.Query("format")
	switch format {
	case "webm", "ogg", "mp4":
	default:
		c.AbortWithError(http.StatusBadRequest, errors.New("unsupported format"))
		return
	}
	hardsub_q := c.Query("hardsub")
	var hardsub bool
	switch hardsub_q {
	case "true":
		hardsub = true
	case "false":
		hardsub = false
	default:
		c.AbortWithError(http.StatusInternalServerError, errors.New("hardsub must be `true` or `false`"))
		return
	}
	overwrite_q := c.Query("hardsub")
	var overwrite bool
	switch overwrite_q {
	case "true":
		overwrite = true
	case "false":
		overwrite = false
	default:
		c.AbortWithError(http.StatusInternalServerError, errors.New("overwrite must be `true` or `false`"))
		return
	}
	var media gin.H
	found := false
	for _, m := range library {
		if m["id"] == c.Query("id") {
			found = true
			media = m
			break
		}
	}
	if !found {
		c.AbortWithError(http.StatusInternalServerError, errors.New("id not found"))
		return
	}
	path := media["path"].(string)
	if filepath.Dir(path) != "/mkv" {
		c.AbortWithError(
			http.StatusInternalServerError,
			fmt.Errorf("invalid file `%s`", path),
		)
		return
	}
	in_basename := filepath.Base(path)
	out_basename := in_basename[:len(in_basename)-len(filepath.Ext(path))] + "." + format
	out := filepath.Join("/", format, out_basename)
	_, err := os.Stat(out)
	if !os.IsNotExist(err) && !overwrite {
		c.AbortWithError(http.StatusConflict, errors.New("file exists and `overwrite` set to `false`"))
	}
	log.Printf("%s", out)
	var cmd *exec.Cmd
	if hardsub {
		cmd = exec.Command(
			"ffmpeg",
			"-y",
			"-i", path,
			"-c:a", "libopus",
			"-vf", fmt.Sprintf("subtitles='%s'", path),
			out,
		)
	} else {
		cmd = exec.Command(
			"ffmpeg",
			"-y",
			"-i", path,
			"-c:a", "libopus",
			out,
		)
	}
	cmd.Stderr = log.Writer()
	cmd.Stdout = log.Writer()
	if err := cmd.Run(); err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}
	c.Status(http.StatusOK)
}
