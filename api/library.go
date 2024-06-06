package main

// import (
// 	"log"
// 	"os"
// 	"path/filepath"

// 	"github.com/fsnotify/fsnotify"
// )

// type Library struct {
// 	subscribers map[chan MediaEvent]struct{}

// 	data_dir string
// 	meta_dir string

// 	store Store
// 	// write-through cache
// 	cache map[string]Media
// }

// type MediaEvent struct {
// 	Id    string
// 	Event string
// }

// func NewLibrary(data_dir string, meta_dir string) (library *Library, err error) {
// 	library = &Library{}
// 	library.store, err = NewSqliteStore(meta_dir)
// 	if err != nil {
// 		return
// 	}
// 	library.data_dir = data_dir
// 	library.meta_dir = meta_dir
// 	library.cache = make(map[string]Media)
// 	library.sync_dir()
// 	library.watch_dir()
// 	return
// }

// func (library *Library) List() (list map[string]Media, err error) {
// 	return library.store.List()
// }

// func (library *Library) Get(id string) (media Media, ok bool, err error) {
// 	return library.store.Get(id)
// }

// func (library *Library) Update(id string, fields map[string]string) (err error) {
// 	return library.store.Update(id, fields)
// }

// func (library *Library) Subscribe() chan MediaEvent {
// 	client := make(chan MediaEvent)
// 	library.subscribers[client] = struct{}{}
// 	return client
// }

// func (library *Library) Unsubscribe(c chan MediaEvent) {
// 	delete(library.subscribers, c)
// 	close(c)
// }

// func (library *Library) add_media(path string) (id string, err error) {
// 	title := filepath.Base(path)
// 	var format string
// 	ext := filepath.Ext(path)
// 	if ext == "" {
// 		format = "unknown"
// 	} else {
// 		format = ext[1:]
// 	}
// 	return library.store.Create(path, Media{
// 		Title:     title,
// 		Format:    format,
// 		Shortname: title,
// 	})
// }

// func (library *Library) sync_dir() {
// 	entries, err := os.ReadDir(library.data_dir)
// 	if err != nil {
// 		log.Fatalf("%s", err)
// 	}
// 	for _, entry := range entries {
// 		log.Printf("entry: %s", entry)
// 		if !entry.Type().IsRegular() {
// 			log.Printf("not a regular file: %s", entry.Name())
// 			continue
// 		}
// 		library.add_media(entry.Name())
// 	}
// }

// func (library *Library) watch_dir() {
// 	watcher, err := fsnotify.NewWatcher()
// 	if err != nil {
// 		log.Fatalf("%s", err)
// 	}
// 	if err := watcher.Add(library.data_dir); err != nil {
// 		log.Fatal(err)
// 	}
// 	library.subscribers = make(map[chan MediaEvent]struct{})
// 	go func() {
// 		for e := range watcher.Events {
// 			if e.Has(fsnotify.Create) {
// 				log.Printf("new media %s", e.Name)
// 				_, base := filepath.Split(e.Name)
// 				id, err := library.add_media(base)
// 				if err != nil {
// 					log.Printf("failed to add media: %s", err)
// 				} else {
// 					for c := range library.subscribers {
// 						c <- MediaEvent{
// 							Id:    id,
// 							Event: "create",
// 						}
// 					}
// 				}
// 			}
// 		}
// 	}()
// 	go func() {
// 		for err := range watcher.Errors {
// 			log.Println("fsnotify error:", err)
// 		}
// 	}()
// }
