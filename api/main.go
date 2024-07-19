package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
)

type Collection interface {
	List() ([]Item, error)
	Create(string, map[string]string) (string, error)
	Get(id string) (Item, bool, error)
	Update(id string, field string, value string) (bool, error)
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

	router := gin.Default()
	add_collection(router, "videos")
	add_collection(router, "images")
	log.Fatal(router.Run())
}

func add_collection(router *gin.Engine, name string) {
	coll := NewMemCollection()
	group := router.Group(name)

	group.GET("/", func(c *gin.Context) {
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

	group.POST("/", func(c *gin.Context) {
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
