package main

import (
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/fsnotify/fsnotify"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

type media struct {
	id    string
	title string
	path  string
}

var media_library []gin.H = make([]gin.H, 0)

func main() {
	// log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	// ck_env_vars()
	watcher := setup_watcher()
	defer watcher.Close()

	watch("/mkv", watcher)
	watch("/webm", watcher)

	router := gin.Default()
	router.GET("/media_library", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"media": media_library})
	})
	log.Fatal(router.Run())
}

func setup_watcher() *fsnotify.Watcher {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		log.Fatal(err)
	}

	go func() {
		for event := range watcher.Events {
			log.Println("event:", event)
			if event.Has(fsnotify.Create) {
				add_media(media{
					id:    uuid.New().String(),
					title: filepath.Base(event.Name),
					path:  event.Name,
				})
			}
		}
	}()

	go func() {
		for err := range watcher.Errors {
			log.Println("error:", err)
		}
	}()

	return watcher
}

func watch(dir string, watcher *fsnotify.Watcher) {
	if err := watcher.Add(dir); err != nil {
		log.Fatal(err)
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		log.Fatalf("%s", err)
	}
	for _, entry := range entries {
		if !entry.Type().IsRegular() {
			log.Printf("not a regular file: %s", entry.Name())
			continue
		}
		add_media(media{
			id:    uuid.New().String(),
			title: entry.Name(),
			path:  filepath.Join(dir, entry.Name()),
		})
	}
}

func add_media(m media) {
	media_library = append(
		media_library,
		gin.H{
			"title":     m.title,
			"id":        m.id,
			"path":      "/media/" + m.path,
			"filetype":  filepath.Ext(m.path)[1:],
			"shortname": m.title,
		},
	)
}

func convert(c *gin.Context) {
	path, err := filepath.EvalSymlinks(filepath.Join("/mkv", c.Param("path")))
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}
	if filepath.Dir(path) != "/mkv" {
		c.AbortWithError(http.StatusInternalServerError, errors.New("invalid file"))
		return
	}
	in_basename := filepath.Base(path)
	out_basename := in_basename[:len(in_basename)-len(filepath.Ext(path))] + ".webm"
	out := filepath.Join("/webm", out_basename)
	log.Printf("%s", out)
	cmd := exec.Command(
		"ffmpeg",
		"-y",
		"-i", path,
		"-vf", fmt.Sprintf("subtitles='%s'", path),
		"-c:a", "libopus",
		out,
	)
	cmd.Stderr = log.Writer()
	cmd.Stdout = log.Writer()
	if err := cmd.Run(); err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}
	c.Status(http.StatusOK)
}
