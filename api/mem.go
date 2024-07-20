package main

import (
	"strings"
	"sync"

	"github.com/google/uuid"
)

type MemCollection struct {
	items map[string]Item
	mu    sync.Mutex
}

func NewMemCollection() *MemCollection {
	return &MemCollection{items: make(map[string]Item)}
}

func (coll *MemCollection) Create(url string, title string, format string) (string, error) {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	if id, err := uuid.NewRandom(); err != nil {
		return "", err
	} else {
		id := id.String()
		coll.items[id] = Item{Url: url, Title: title, Format: format}
		return id, nil
	}
}

func (coll *MemCollection) List() (map[string]Item, error) {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	return coll.items, nil
}

func (coll *MemCollection) Get(id string) (Item, bool, error) {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	item, ok := coll.items[id]
	return item, ok, nil
}

func (coll *MemCollection) Update(id string, field string, value string) (bool, error) {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	if item, ok := coll.items[id]; ok {
		switch strings.ToLower(field) {
		case "title":
			item.Title = value
		case "format":
			item.Format = value
		}
		return true, nil
	} else {
		return false, nil
	}
}

func (coll *MemCollection) Drop(id string) error {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	delete(coll.items, id)
	return nil
}
