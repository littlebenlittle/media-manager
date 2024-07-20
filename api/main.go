package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path"
	"strings"

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
	index := func(p string) {
		title, format := extract_title_format(p)
		if format == "unknown" {
			title, format = ffprobe(p)
		}
		url := fmt.Sprintf("%s%s", MEDIA_SERVER_URL, p[len("/data"):])
		id, err := media.Create(url, title, format)
		if err != nil {
			log.Print(err)
		}
		path_to_id[p] = id
	}
	walk("/data", index)
	watch("/data", func(ev fsnotify.Event) {
		switch ev.Op {
		case fsnotify.Create:
			log.Printf("create %s", ev.Name)
			index(ev.Name)
		case fsnotify.Rename, fsnotify.Remove:
			log.Printf("remove %s", ev.Name)
			id := path_to_id[ev.Name]
			media.Drop(id)
		}
	})

	router := gin.Default()
	add_collection(router, media, "media")

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

func walk(dir string, f func(string)) {
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
	title := base[:len(ext)]
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
