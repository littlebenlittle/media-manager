package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path"
	"strings"

	"benlittle.dev/media-manager/protocol"
	"github.com/fsnotify/fsnotify"
	"nhooyr.io/websocket"
	"nhooyr.io/websocket/wsjson"
)

type Data = protocol.Data
type Association = protocol.Association
type ID = protocol.ID
type ServerMsg = protocol.ServerMsg
type ClientMsg = protocol.ClientMsg
type Event = protocol.Event

var SERVER_PUBLIC_URL = os.Getenv("SERVER_PUBLIC_URL")

type Path string

type Metadata struct {
	Title  string
	Format string
	URL    string
}

type InMsg struct {
	Sync *struct{}
}

type OutMsg struct {
	Exists  *Metadata
	Removed *Metadata
}

func validate_env() {
	if SERVER_PUBLIC_URL == "" {
		log.Fatalf("please set SERVER_PUBLIC_URL")
	}
}

func main() {
	validate_env()
	media := make(map[Path]Metadata)
	subs := make(map[chan OutMsg]struct{})
	add_sub := make(chan chan OutMsg)
	rm_sub := make(chan chan OutMsg)
	add_path := make(chan Path)
	rm_path := make(chan Path)
	go walk("/data", add_path)
	go watch("/data", add_path, rm_path)
	go ListenWS("0.0.0.0:80", func(in chan InMsg, out chan OutMsg) {
		add_sub <- out
		for msg := range in {
			if msg.Sync != nil {
				for _, meta := range media {
					out <- OutMsg{Exists: &meta}
				}
			}
		}
		rm_sub <- out
	})
	for {
		select {
		case sub := <-add_sub:
			subs[sub] = struct{}{}
		case sub := <-rm_sub:
			delete(subs, sub)
			close(sub)
		case path := <-add_path:
			meta := probe(path)
			media[path] = meta
			for ch := range subs {
				ch <- OutMsg{Exists: &meta}
			}
		case path := <-rm_path:
			meta := media[path]
			delete(media, path)
			for ch := range subs {
				ch <- OutMsg{Removed: &meta}
			}
		}
	}
}

func probe(path Path) (meta Metadata) {
	meta.Title, meta.Format = extract_title_format(path)
	if meta.Format == "unknown" {
		meta.Title, meta.Format = ffprobe(path)
	}
	meta.URL = fmt.Sprintf("%s%s", SERVER_PUBLIC_URL, path[len("/data"):])
	return
}

func walk(dir string, add_path chan Path) {
	if entries, err := os.ReadDir(dir); err != nil {
		log.Print(err)
	} else {
		for _, entry := range entries {
			path := path.Join(dir, entry.Name())
			t := entry.Type()
			if t.IsDir() {
				walk(path, add_path)
			}
			add_path <- Path(path)
		}
	}
}

func watch(dir string, add_path chan Path, rm_path chan Path) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		log.Fatalf("%s", err)
	}
	watcher.Add(dir)
	for ev := range watcher.Events {
		switch ev.Op {
		case fsnotify.Create:
			add_path <- Path(ev.Name)
		case fsnotify.Remove, fsnotify.Rename:
			rm_path <- Path(ev.Name)
		}
	}
}

func extract_title_format(p Path) (string, string) {
	base := path.Base(string(p))
	ext := path.Ext(string(p))
	title := base[:len(base)-len(ext)]
	if ext == "" {
		return title, "unknown"
	} else {
		return title, ext[1:]
	}
}

func ffprobe(p Path) (string, string) {
	cmd := exec.Command(
		"ffprobe",
		"-v", "quiet",
		"-print_format", "json",
		"-show_format",
		string(p),
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

func ListenWS(listen string, handler func(chan InMsg, chan OutMsg)) {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		log.Printf("new client: %s", r.RemoteAddr)
		c, err := websocket.Accept(w, r, nil)
		if err != nil {
			log.Fatalf("%s", err)
		}
		defer c.CloseNow()
		ctx, cancel := context.WithCancel(r.Context())
		defer cancel()
		in := make(chan InMsg)
		out := make(chan OutMsg)
		go func() {
			for {
				var msg InMsg
				if err := wsjson.Read(ctx, c, &msg); err == nil {
					in <- msg
				} else {
					log.Printf("%s", err)
					break
				}
			}
		}()
		go func() {
			for r := range out {
				if err := wsjson.Write(ctx, c, r); err != nil {
					log.Printf("%s", err)
				}
			}
		}()
		handler(in, out)
	})
	log.Printf("listen: %s", listen)
	log.Fatal(http.ListenAndServe(listen, nil))
}
