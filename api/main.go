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
	List() ([]Item, error)
	Create(string, map[string]string) (string, error)
	Get(id string) (Item, bool, error)
	Update(id string, field string, value string) (bool, error)
	Drop(id string) error
}

type Item struct {
	ID  string
	Url string
	// If Meta contains a field "id", it will
	// be overwritten in responses to clients
	Meta map[string]string
}

func main() {
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	if MEDIA_SERVER_URL == "" {
		log.Fatalf("please set env var MEDIA_SERVER_URL")
	}

	path_to_id := make(map[string]string)
	videos := NewMemCollection()
	images := NewMemCollection()
	index := func(p string) {
		title, format := extract_title_format(p)
		if format == "unknown" {
			format = ffprobe(p)
		}
		fields := map[string]string{
			"title":  title,
			"format": format,
		}
		url := fmt.Sprintf("%s%s", MEDIA_SERVER_URL, p[len("/data"):])
		var coll Collection
		switch format {
		case "mkv", "mp4", "ogg", "webm":
			coll = videos
		case "jpg", "jpeg", "png", "webp":
			coll = images
		default:
			log.Printf("unsupported format, skipping: %s", p)
			return
		}
		id, err := coll.Create(url, fields)
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
			videos.Drop(id)
			images.Drop(id)
		}
	})

	router := gin.Default()
	add_collection(router, videos, "videos")
	add_collection(router, images, "images")

	log.Fatal(router.Run())
}

func add_collection(router *gin.Engine, coll Collection, name string) {
	group := router.Group(name)

	group.GET("", func(c *gin.Context) {
		if items, err := coll.List(); err == nil {
			res := make([]map[string]string, len(items))
			for i, item := range items {
				res[i] = item.Meta
				res[i]["id"] = item.ID
			}
			c.JSON(http.StatusOK, items)
		} else {
			c.Error(err)
			c.Status(http.StatusInternalServerError)
		}
	})

	group.POST("", func(c *gin.Context) {
		var item struct {
			Url  string
			Meta map[string]string
		}
		if err := c.BindJSON(&item); err != nil {
			c.String(http.StatusBadRequest, "invalid JSON: %s", err)
		} else {
			if id, err := coll.Create(item.Url, item.Meta); err == nil {
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

func ffprobe(p string) string {
	cmd := exec.Command(
		"ffprobe",
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
			FormatName string `json:"format_name"`
		}
	}
	if err := json.Unmarshal(buf.Bytes(), &v); err != nil {
		log.Fatal(err)
	}
	switch strings.Split(v.Format.FormatName, ",")[0] {
	case "matroska":
		return "mkv"
	}
	return "unknown"
}
