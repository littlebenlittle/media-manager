package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/fsnotify/fsnotify"
	"github.com/google/uuid"
)

var ErrNotImplemented error = errors.New("not implemented")

// Mitigates "oopsing" strings that should not be considered keys.
// Can always `NewIDFromString`, but requires developer to be
// explicit.
type ID struct {
	repr string
}

func NewID() ID {
	return ID{repr: uuid.New().String()}
}

// TODO define a regex for the string to match
func NewIDFromString(s string) (ID, error) {
	return ID{repr: s}, nil
}

func (id *ID) String() string {
	return id.repr
}

func (id *ID) MarshalJSON() ([]byte, error) {
	return json.Marshal(&id.repr)
}

func (id *ID) UnmarshalJSON(data []byte) error {
	return json.Unmarshal(data, &id.repr)
}

type Resolve[T any] interface {
	Resolve(other T) T
}

type Media struct {
	Title     string
	Format    string
	Shortname string
	Path      string
}

// Always prefer other fields unless they are zero
func (m Media) Resolve(other Media) (media Media) {
	media = other
	if media.Title == "" {
		media.Title = m.Title
	}
	if media.Format == "" {
		media.Format = m.Format
	}
	if media.Shortname == "" {
		media.Shortname = m.Shortname
	}
	if media.Path == "" {
		media.Path = m.Path
	}
	return
}

func (m *Media) Merge(update map[string]string) error {
	for k, v := range update {
		switch k {
		case "format":
			m.Format = v
		case "title":
			m.Title = v
		case "shortname":
			m.Shortname = v
		default:
			return fmt.Errorf("unacceptable field `%s`", k)
		}
	}
	return nil
}

type Storage interface {
	List() (items map[string]string, err error)
	Get(key string) (val string, ok bool)
	Assign(key string, val string)
	Delete(key string)
}

type StorageEventKind = int

const (
	Assign StorageEventKind = iota
	Delete
)

type StorageEvent[T any] struct {
	ID
	Val  T
	Kind StorageEventKind
}

// TODO comparable should be replaced with
// a trait for resolving conflicting values
type Collection[T any] struct {
	kv     Storage
	prefix string
	subs   map[chan StorageEvent[T]]struct{}
}

func (c *Collection[T]) List() (items map[ID]T, err error) {
	kvs, err := c.kv.List()
	if err != nil {
		return
	}
	items = make(map[ID]T)
	n_errs := 0
	for k, v := range kvs {
		if strings.HasPrefix(k, c.prefix) {
			var t T
			if err := json.Unmarshal([]byte(v), &t); err != nil {
				log.Printf("failed to unmarshal `%s` `%s`: %s", k, v, err)
				n_errs++
			}
			items[ID{repr: k[len(c.prefix):]}] = t
		}
	}
	if n_errs > 0 {
		err = errors.New("failed to unmarshal some data")
	}
	return
}

func (c *Collection[T]) Get(id ID) (val T, ok bool) {
	items, _ := c.kv.List()
	for i := range items {
		log.Printf("%s", i)
	}
	var v string
	v, ok = c.kv.Get(c.prefix + id.repr)
	if ok {
		if err := json.Unmarshal([]byte(v), &val); err != nil {
			log.Printf("failed to unmarshal `%s`: %s", id.repr, err)
		}
	}
	return
}

func (c *Collection[T]) Assign(id ID, val T) {
	if v, err := json.Marshal(val); err != nil {
		log.Printf("failed to marshal: %s", err)
	} else {
		c.kv.Assign(c.prefix+id.repr, string(v))
	}
}

func (c *Collection[T]) Delete(id ID) {
	c.kv.Delete(c.prefix + id.repr)
}

func (c *Collection[T]) Subscribe() (events chan StorageEvent[T]) {
	events = make(chan StorageEvent[T])
	c.subs[events] = struct{}{}
	return events
}

func (c *Collection[T]) Unsubscribe(events chan StorageEvent[T]) {
	delete(c.subs, events)
	close(events)
}

// func (c *Collection[T]) Sync(other map[ID]T) (unknown []ID, missing map[ID]T, err error) {
// 	var mds map[ID]T
// 	if mds, err = c.List(); err != nil {
// 		return
// 	}
// 	missing = make(map[ID]T, 0)
// 	for id, server_t := range mds {
// 		if client_t, ok := other[id]; ok {
// 			c.Assign(id, server_t.Resolve(client_t))
// 			delete(other, id)
// 		} else {
// 			missing[id] = server_t
// 		}
// 	}
// 	unknown = make([]ID, 0)
// 	for id := range other {
// 		unknown = append(unknown, id)
// 	}
// 	return
// }

type Manager struct {
	kv Storage

	Media *Collection[Media]
	// Jobs  *Collection[Job]
}

func NewManager() (m *Manager, err error) {
	m = &Manager{}
	m.kv = NewMemStorage()
	m.Media = &Collection[Media]{
		kv:     m.kv,
		subs:   make(map[chan StorageEvent[Media]]struct{}),
		prefix: "metadata/",
	}
	return
}

// TODO this _should_ lock the FS path from other services
// like `tusd` and `transmission`
func (m *Manager) SyncFS(path string) (err error) {
	// assume FS lock acquired
	entries, err := os.ReadDir(path)
	if err != nil {
		return err
	}
	paths := make(map[string]struct{})
	for _, entry := range entries {
		if !entry.Type().IsRegular() {
			// TODO handle directories, which can happen if
			// `transmission` downloads a batch of files or
			// I decide to include an un-archiving job
			log.Printf("not a regular file: %s", entry.Name())
			continue
		}
		paths[filepath.Join(path, entry.Name())] = struct{}{}
	}
	var mds map[ID]Media
	if mds, err = m.Media.List(); err != nil {
		return
	}
	for id, md := range mds {
		if _, ok := paths[md.Path]; ok {
			delete(paths, md.Path)
		} else {
			log.Printf("file does not exist: %s", md.Path)
			m.Media.Delete(id)
		}
	}
	for p := range paths {
		m.Media.Assign(NewID(), guess_metadata_from_path(p))
	}
	// assume FS lock is released
	return
}

func (m *Manager) Watch(path string) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		log.Fatalf("%s", err)
	}
	if err := watcher.Add(path); err != nil {
		log.Fatal(err)
	}
	go func() {
		for e := range watcher.Events {
			if e.Has(fsnotify.Create) {
				log.Printf("new media %s", e.Name)
				id := NewID()
				val := guess_metadata_from_path(e.Name)
				m.Media.Assign(id, val)
				for c := range m.Media.subs {
					c <- StorageEvent[Media]{
						ID:   id,
						Val:  val,
						Kind: Assign,
					}
				}
			}
		}
	}()
	go func() {
		for err := range watcher.Errors {
			log.Println("fsnotify error:", err)
		}
	}()
}

func guess_metadata_from_path(path string) Media {
	title := filepath.Base(path)
	var format string
	ext := filepath.Ext(path)
	if ext == "" {
		format = "unknown"
	} else {
		format = ext[1:]
	}
	return Media{
		Title:     title,
		Format:    format,
		Shortname: title,
		Path:      path,
	}
}

// type LogEntry struct {
// 	message string
// }

// type Job struct {
// 	logs chan LogEntry
// 	subs map[chan LogEntry]struct{}

// 	ID      string
// 	Kind    string
// 	Options any
// 	Error   string
// }

// func (m *Job) Subscribe() (entries chan LogEntry) {
// 	entries = make(chan LogEntry)
// 	m.subs[entries] = struct{}{}
// 	return
// }

// func (m *Job) Unsubscribe(entries chan LogEntry) {
// 	delete(m.subs, entries)
// 	close(entries)
// }

// func (m *Manager) ListJobs() []Job {
// 	jobs := make([]Job, len(m.Jobs))
// 	i := 0
// 	for _, j := range m.Jobs {
// 		jobs[i] = Job{
// 			Kind:    j.Kind,
// 			Options: j.Options,
// 			Error:   j.Error,
// 		}
// 	}
// 	return jobs
// }

// func (m *JobsManager) Start(job Job) error {
// 	switch strings.ToLower(job.Kind) {
// 	case "convert":
// 		switch opts := job.Options.(type) {
// 		case ConvertOptions:
// 			media, ok, err := m.kv.Get(opts.MediaID)
// 			if err == nil && ok {
// 				var err_ch chan error
// 				job.logs, err_ch, err = convert(media.Path, opts.Format, opts.Hardsub, opts.Overwrite)
// 				if err == nil {
// 					go func() { job.Error = fmt.Sprintf("%s", <-err_ch) }()
// 				}
// 				m.register_job(job)
// 			}
// 			return err
// 		}
// 	default:
// 		return errors.New("unknown job kind")
// 	}
// 	return nil
// }

// func (m *JobsManager) register_job(job Job) {
// 	job.ID = uuid.New().String()
// 	m.jobs[job.ID] = &job
// 	for c := range m.subs {
// 		c <- job
// 	}
// }

// func (m *JobsManager) Get(id string) (Job, bool) {
// 	job, ok := m.jobs[id]
// 	return *job, ok
// }

// func (m *JobsManager) Subscribe() (jobs chan Job) {
// 	jobs = make(chan Job)
// 	m.subs[jobs] = struct{}{}
// 	return
// }

// func (m *JobsManager) Unsubscribe(jobs chan Job) {
// 	delete(m.subs, jobs)
// 	close(jobs)
// }
