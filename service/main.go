package main

import (
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/gin-gonic/gin"
)

func main() {
	// log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	// ck_env_vars()
	router := gin.Default()
	router.GET("/mkv", func(c *gin.Context) { get_media("/mkv", c) })
	router.GET("/webm", func(c *gin.Context) { get_media("/webm", c) })
	router.GET("/convert/:path", convert)
	log.Fatal(router.Run())
}

func get_media(dir string, c *gin.Context) {
	media_list := []gin.H{}
	entries, err := os.ReadDir(dir)
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}
	for _, entry := range entries {
		if !entry.Type().IsRegular() {
			log.Printf("not a regular file: %s", entry.Name())
			continue
		}
		media_list = append(media_list, gin.H{"title": entry.Name()})
	}
	c.JSON(http.StatusOK, media_list)
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
