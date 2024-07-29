package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path"
	"strings"
	"sync"
	"time"

	"github.com/fsnotify/fsnotify"
	"github.com/gin-gonic/gin"
)

var MEDIA_SERVER_URL = os.Getenv("MEDIA_SERVER_URL")

type Collection interface {
	List() (map[string]Item, error)
	Create(string, string, string) (string, error)
	Get(id string) (Item, bool, error)
	Update(id string, field string, value string) (bool, error)
	Drop(id string) error
}

type Item struct {
	Url    string
	Title  string
	Format string
}

func main() {
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	if MEDIA_SERVER_URL == "" {
		log.Fatalf("please set env var MEDIA_SERVER_URL")
	}

	path_to_id := make(map[string]string)
	media := NewMemCollection()
	index := func(p string) gin.H {
		title, format := extract_title_format(p)
		if format == "unknown" {
			title, format = ffprobe(p)
		}
		url := fmt.Sprintf("%s%s", MEDIA_SERVER_URL, p[len("/data"):])
		id, err := media.Create(url, title, format)
		if err != nil {
			log.Print(err)
			return nil
		}
		path_to_id[p] = id
		return gin.H{
			"id":     id,
			"title":  title,
			"format": format,
			"url":    url,
		}

	}
	walk("/data", index)

	clients := make(map[chan gin.H]struct{})
	mu := sync.Mutex{}
	watch("/data", func(ev fsnotify.Event) {
		switch ev.Op {
		case fsnotify.Create:
			log.Printf("create %s", ev.Name)
			item := index(ev.Name)
			mu.Lock()
			log.Printf("clients: %d", len(clients))
			for client := range clients {
				client <- gin.H{"Create": []any{item["id"], item}}
			}
			mu.Unlock()
		case fsnotify.Rename, fsnotify.Remove:
			log.Printf("remove %s", ev.Name)
			id := path_to_id[ev.Name]
			if err := media.Drop(id); err == nil {
				for client := range clients {
					client <- gin.H{"Forget": id}
				}
			} else {
				log.Print(err)
			}
		}
	})

	router := gin.Default()
	add_collection(router, media, "media")

	router.GET("/events", func(c *gin.Context) {
		c.Writer.Header().Set("Content-Type", "text/event-stream")
		c.Writer.Header().Set("Cache-Control", "no-cache")
		c.Writer.Header().Set("Connection", "keep-alive")
		c.Writer.Header().Set("Transfer-Encoding", "chunked")
		c.Writer.Header().Set("X-Accel-Buffering", "no")
		ch := make(chan gin.H)
		log.Printf("new client")
		mu.Lock()
		clients[ch] = struct{}{}
		mu.Unlock()
		// defer func() {
		// 	log.Printf("closing client channel")
		// 	mu.Lock()
		// 	close(ch)
		// 	delete(clients, ch)
		// 	mu.Unlock()
		// }()
		t := make(chan struct{})
		ev := make(chan gin.H)
		sync := true
		c.Stream(func(_ io.Writer) bool {
			if sync {
				log.Printf("syncing client")
				if items, err := media.List(); err == nil {
					res := make([]map[string]string, len(items))
					i := 0
					for id, item := range items {
						res[i] = map[string]string{
							"id":     id,
							"url":    item.Url,
							"title":  item.Title,
							"format": item.Format,
						}
						i++
					}
					c.SSEvent("message", gin.H{"Sync": res})
				}
				sync = false
				return true
			}
			go func() {
				time.Sleep(30 * time.Second)
				t <- struct{}{}
			}()
			go func() {
				if event, ok := <-ch; ok {
					ev <- event
				} else {
					ev <- nil
				}
			}()
			select {
			case <-t:
				// keeps proxy server from closing the connection
				log.Printf("pinging client")
				c.SSEvent("ping", nil)
				return true
			case event := <-ev:
				if ev == nil {
					return false
				} else {
					log.Printf("sending event to client")
					c.SSEvent("message", event)
					log.Printf("event sent")
					return true
				}
			}
		})
	})

	log.Fatal(router.Run())
}

func add_collection(router *gin.Engine, coll Collection, name string) {
	group := router.Group(name)

	group.GET("", func(c *gin.Context) {
		if items, err := coll.List(); err == nil {
			res := make([]map[string]string, len(items))
			i := 0
			for id, item := range items {
				res[i] = map[string]string{
					"id":     id,
					"url":    item.Url,
					"title":  item.Title,
					"format": item.Format,
				}
				i++
			}
			c.JSON(http.StatusOK, res)
		} else {
			c.Error(err)
			c.Status(http.StatusInternalServerError)
		}
	})

	group.POST("", func(c *gin.Context) {
		var item Item
		if err := c.BindJSON(&item); err != nil {
			c.String(http.StatusBadRequest, "invalid JSON: %s", err)
		} else {
			if id, err := coll.Create(item.Url, item.Title, item.Format); err == nil {
				c.String(http.StatusAccepted, "%s", id)
			} else {
				c.Error(err)
				c.Status(http.StatusInternalServerError)
			}
		}
	})

	group.GET("/:id", func(c *gin.Context) {
		id := c.Param("id")
		if item, ok, err := coll.Get(id); ok {
			c.Redirect(http.StatusTemporaryRedirect, item.Url)
		} else if err == nil {
			c.Status(http.StatusNotFound)
		} else {
			c.Error(err)
			c.Status(http.StatusInternalServerError)
		}
	})

	group.PATCH("/:id", func(c *gin.Context) {
		id := c.Param("id")
		field := c.Query("f")
		value := c.Query("v")
		if ok, err := coll.Update(id, field, value); ok {
			c.Status(http.StatusOK)
		} else if err == nil {
			c.Status(http.StatusNotFound)
		} else {
			c.Error(err)
			c.Status(http.StatusInternalServerError)
		}
	})
}

func walk(dir string, f func(string) gin.H) {
	if entries, err := os.ReadDir(dir); err != nil {
		log.Print(err)
	} else {
		for _, entry := range entries {
			path := path.Join(dir, entry.Name())
			t := entry.Type()
			if t.IsDir() {
				walk(path, f)
			}
			f(path)
		}
	}
}

func watch(dir string, f func(fsnotify.Event)) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		log.Fatal(err)
	}
	watcher.Add(dir)
	go func() {
		for {
			f(<-watcher.Events)
		}
	}()
}

func extract_title_format(p string) (string, string) {
	base := path.Base(p)
	ext := path.Ext(p)
	title := base[:len(base)-len(ext)]
	if ext == "" {
		return title, "unknown"
	} else {
		return title, ext[1:]
	}
}

func ffprobe(p string) (string, string) {
	cmd := exec.Command(
		"ffprobe",
		"-v", "quiet",
		"-print_format", "json",
		"-show_format",
		p,
	)
	cmd.Stderr = log.Writer()
	buf := new(bytes.Buffer)
	cmd.Stdout = buf
	if err := cmd.Run(); err != nil {
		log.Fatal(err)
	}
	var v struct {
		Format struct {
			FileName   string `json:"filename"`
			FormatName string `json:"format_name"`
			Tags       map[string]string
		}
	}
	if err := json.Unmarshal(buf.Bytes(), &v); err != nil {
		log.Fatal(err)
	}
	var title string
	title, ok := v.Format.Tags["title"]
	if !ok {
		title = path.Base(v.Format.FileName)
	}
	if strings.HasPrefix(v.Format.FormatName, "matroska") {
		return title, "mkv"
	} else if strings.HasPrefix(v.Format.FormatName, "mov,mp4") {
		return title, "mp4"
	}
	return title, "unknown"
}
