package main

import (
	"fmt"
	"log"
	"math/rand"
	"net/http"

	"github.com/gin-gonic/gin"
)

type Store interface{}

type Collection struct{}

func (coll *Collection) List() ([]Item, error)
func (coll *Collection) Get(id string) (Item, bool, error)
func (coll *Collection) Update(id string, field string, value string) (bool, error)

type Item struct {
	ID  string
	Url string
	// If Meta contains a field "id", it will
	// be overwritten in responses to clients
	Meta map[string]string
}

func coll(store Store) func(*gin.Context)

func NewMemStore() MemStore

type MemStore struct{}

func logerr(err error) string {
	id := fmt.Sprintf("%4x", rand.Int31())
	log.Printf("(%s) %s", id, err)
	return id
}

func main() {
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)

	store := NewMemStore()
	router := gin.Default()

	collections := router.Group("/:coll", coll(store))
	collections.GET("", func(c *gin.Context) {
		coll := c.MustGet("coll").(Collection)
		if items, err := coll.List(); err == nil {
			res := make([]map[string]string, len(items))
			for i, item := range items {
				res[i] = item.Meta
				res[i]["id"] = item.ID
			}
			c.JSON(http.StatusOK, items)
		} else {
			c.String(http.StatusInternalServerError, "errid: %s", logerr(err))
		}
	})

	collections.GET("/:id", func(c *gin.Context) {
		coll := c.MustGet("coll").(Collection)
		id := c.Param("id")
		if item, ok, err := coll.Get(id); ok {
			c.Redirect(http.StatusTemporaryRedirect, item.Url)
		} else if err == nil {
			c.Status(http.StatusNotFound)
		} else {
			c.String(http.StatusInternalServerError, "errid: %s", logerr(err))
		}
	})

	collections.PUT("/:id", func(c *gin.Context) {
		coll := c.MustGet("coll").(Collection)
		id := c.Param("id")
		field := c.Query("f")
		value := c.Query("v")
		if ok, err := coll.Update(id, field, value); ok {
			c.Status(http.StatusOK)
		} else if err == nil {
			c.Status(http.StatusNotFound)
		} else {
			c.String(http.StatusInternalServerError, "errid: %s", logerr(err))
		}
	})

	log.Fatal(router.Run())
}
