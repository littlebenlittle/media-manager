package main

import (
	"sync"

	"github.com/google/uuid"
)

type MemCollection struct {
	items map[string]Item
	mu    sync.Mutex
}

func NewMemCollection() *MemCollection {
	return &MemCollection{}
}

func (coll *MemCollection) Create(url string, meta map[string]string) (string, error) {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	if id, err := uuid.NewRandom(); err != nil {
		return "", err
	} else {
		id := id.String()
		coll.items[id] = Item{ID: id, Url: url, Meta: meta}
		return id, nil
	}
}

func (coll *MemCollection) List() ([]Item, error) {
	coll.mu.Lock()
	defer coll.mu.Unlock()
	list := make([]Item, len(coll.items))
	i := 0
	for _, item := range coll.items {
		list[i] = item
		i++
	}
	return list, nil
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
		item.Meta[field] = value
		return true, nil
	} else {
		return false, nil
	}
}
