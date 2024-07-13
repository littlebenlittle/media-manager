package main

import (
	"io"
	"log"
	"net/http"
	"path/filepath"
	"time"

	"github.com/gin-gonic/gin"
)

func main() {
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)

	m, err := NewManager()
	if err != nil {
		log.Fatalf("failed to initialize manager: %s", err)
	}

	dir := "/data"
	if err := m.SyncFS(dir); err != nil {
		log.Panicf("could not sync manager with fs: %s", err)
	}
	m.Watch(dir)

	router := gin.Default()

	router.GET("/media", origin, func(c *gin.Context) {
		if media, err := m.Media.List(); err != nil {
			log.Printf("could not list media: %s", err)
			c.Status(http.StatusInternalServerError)
		} else {
			res := to_client_media_map(dir, c.MustGet("origin").(string), media)
			c.JSON(http.StatusOK, res)
		}
	})

	router.PUT("/media/:id", find_id(), find_media(m), func(c *gin.Context) {
		id := c.MustGet("id").(ID)
		media := c.MustGet("media").(Media)
		field := c.Query("f")
		value := c.Query("v")
		switch field {
		case "title":
			media.Title = value
		case "format":
			media.Format = value
		default:
			c.String(http.StatusBadRequest, "invalid value for query param `f`")
		}
		m.Media.Assign(id, media)
	})

	router.GET("/events/media", sse_headers, func(c *gin.Context) {
		events := m.Media.Subscribe()
		defer m.Media.Unsubscribe(events)
		c.Stream(func(_ io.Writer) bool {
			select {
			case event, ok := <-events:
				if ok {
					c.SSEvent("message", event)
				}
				return ok
			case <-timeout(60 * time.Second):
				c.SSEvent("keepalive", nil)
				return true
			}
		})
	})

	router.POST("/sync/media", origin, func(c *gin.Context) {
		var update map[string]map[string]string
		if err := c.BindJSON(&update); err != nil {
			c.String(http.StatusBadRequest, "%s", err)
			return
		}
		var media map[ID]Media
		if media, err = m.Media.List(); err != nil {
			log.Printf("%s", err)
			c.Status(http.StatusInternalServerError)
			return
		}
		missing := make(map[string]gin.H, 0)
		for id, server_t := range media {
			log.Printf("id: %s", id.String())
			if client_t, ok := update[id.String()]; ok {
				server_t.Merge(client_t)
				m.Media.Assign(id, server_t)
				delete(update, id.String())
			} else {
				origin := c.MustGet("origin").(string)
				missing[id.String()] = to_client_media(dir, origin, server_t)
			}
		}
		unknown := make([]string, 0)
		for id := range update {
			unknown = append(unknown, id)
		}
		c.JSON(http.StatusCreated, gin.H{
			"unknown": unknown,
			"missing": missing,
		})
	})

	// router.GET("/jobs", func(c *gin.Context) {
	// 	if media, err := m.Jobs.List(); err != nil {
	// 		c.JSON(http.StatusOK, media)
	// 	} else {
	// 		log.Printf("could not list jobs: %s", err)
	// 		c.Status(http.StatusInternalServerError)
	// 	}
	// })

	// router.POST("/jobs", func(c *gin.Context) {
	// 	var job Job
	// 	if err := c.BindJSON(&job); err != nil {
	// 		c.String(http.StatusBadRequest, "%s", err)
	// 		return
	// 	} else {
	// 		if err := m.Jobs().Start(job); err != nil {
	// 			c.Status(http.StatusInternalServerError)
	// 		} else {
	// 			c.Status(http.StatusOK)
	// 		}
	// 	}
	// })

	// router.GET("/jobs/subscribe", func(c *gin.Context) {
	// 	jobs := m.Jobs().Subscribe()
	// 	defer m.Jobs().Unsubscribe(jobs)
	// 	c.Stream(func(_ io.Writer) bool {
	// 		select {
	// 		case job, ok := <-jobs:
	// 			if ok {
	// 				c.SSEvent("message", job)
	// 			}
	// 			return ok
	// 		case <-timeout(60 * time.Second):
	// 			c.SSEvent("keepalive", nil)
	// 			return true
	// 		}
	// 	})
	// })

	// router.GET("/jobs/:id", func(c *gin.Context) {
	// 	job, ok := m.Jobs().Get(c.Param("id"))
	// 	if !ok {
	// 		c.Status(http.StatusNotFound)
	// 		return
	// 	}
	// 	c.JSON(http.StatusOK, job)
	// })

	// router.GET("/jobs/:id/logs", func(c *gin.Context) {
	// 	job, ok := m.Jobs().Get(c.Param("id"))
	// 	if !ok {
	// 		c.Status(http.StatusNotFound)
	// 		return
	// 	}
	// 	messages := job.Subscribe()
	// 	defer job.Unsubscribe(messages)
	// 	c.Stream(func(_ io.Writer) bool {
	// 		select {
	// 		case message, ok := <-messages:
	// 			if ok {
	// 				c.SSEvent("message", message)
	// 			}
	// 			return ok
	// 		case <-timeout(60 * time.Second):
	// 			c.SSEvent("keepalive", nil)
	// 			return true
	// 		}
	// 	})
	// })

	log.Fatal(router.Run())
}

func find_id() func(c *gin.Context) {
	return func(c *gin.Context) {
		id, err := NewIDFromString(c.Param("id"))
		if err != nil {
			c.String(http.StatusBadRequest, "%s", err)
			return
		}
		c.Set("id", id)
	}
}

func find_media(m *Manager) func(c *gin.Context) {
	return func(c *gin.Context) {
		id, _ := c.Get("id")
		media, ok := m.Media.Get(id.(ID))
		if !ok {
			log.Printf("media not found: %s", id)
			c.AbortWithStatus(http.StatusNotFound)
			return
		}
		c.Set("media", media)
	}
}

func sse_headers(c *gin.Context) {
	c.Writer.Header().Set("Content-Type", "text/event-stream")
	c.Writer.Header().Set("Cache-Control", "no-cache")
	c.Writer.Header().Set("Connection", "keep-alive")
	c.Writer.Header().Set("Transfer-Encoding", "chunked")
	c.Writer.Header().Set("X-Accel-Buffering", "no")
}

func timeout(d time.Duration) (t chan struct{}) {
	t = make(chan struct{})
	go func() {
		time.Sleep(d)
		t <- struct{}{}
	}()
	return
}

func map_id_to_string[T any](i map[ID]T) map[string]T {
	r := make(map[string]T, len(i))
	for id, t := range i {
		r[id.String()] = t
	}
	return r
}

func map_string_to_id[T any](i map[string]T) (map[ID]T, error) {
	r := make(map[ID]T, len(i))
	for s, t := range i {
		if id, err := NewIDFromString(s); err != nil {
			return nil, err
		} else {
			r[id] = t
		}
	}
	return r, nil
}

func to_client_media(dir string, origin string, media Media) gin.H {
	path, err := filepath.Rel(dir, media.Path)
	if err != nil {
		log.Panicf("%s", err)
	}
	return gin.H{
		"title":     media.Title,
		"format":    media.Format,
		"shortname": media.Shortname,
		"url":       origin + "/downloads/" + path,
	}
}

func to_client_media_map(dir string, origin string, media map[ID]Media) map[string]gin.H {
	ret := make(map[string]gin.H, len(media))
	for id, m := range media {
		ret[id.String()] = to_client_media(dir, origin, m)
	}
	return ret
}

type Location struct {
	Scheme string
	Host   string
}

func origin(c *gin.Context) {
	host := c.Request.Host
	scheme := c.Request.URL.Scheme
	if scheme == "" {
		scheme = "http"
	}
	c.Set("origin", scheme+"://"+host)
}
