package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
)

func coll(store Store) func(*gin.Context) {
	return func(c *gin.Context) {
		name := c.Param("coll")
		if coll, ok, err := store.Coll(name); ok {
			c.Set("coll", coll)
		} else if err == nil {
			c.AbortWithStatus(http.StatusNotFound)
		} else {
			c.Error(err)
			c.AbortWithStatus(http.StatusInternalServerError)
		}
	}
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
			c.Error(err)
			c.Status(http.StatusInternalServerError)
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
			c.Error(err)
			c.Status(http.StatusInternalServerError)
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
			c.Error(err)
			c.Status(http.StatusInternalServerError)
		}
	})

	log.Fatal(router.Run())
}
