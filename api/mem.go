package main

// import (
// 	"strings"
// 	"sync"

// 	"github.com/google/uuid"
// )

// type MemCollection struct {
// 	items map[string]Item
// 	mu    sync.Mutex
// }

// func NewMemCollection() Collection {
// 	return &MemCollection{items: make(map[string]Item)}
// }

// func (coll *MemCollection) List() (map[string]Item, error) {
// 	coll.mu.Lock()
// 	defer coll.mu.Unlock()
// 	return coll.items, nil
// }

// func (coll *MemCollection) Create(item Item) (string, error) {
// 	coll.mu.Lock()
// 	defer coll.mu.Unlock()
// 	if id, err := uuid.NewRandom(); err != nil {
// 		return "", err
// 	} else {
// 		id := id.String()
// 		coll.items[id] = item
// 		return id, nil
// 	}
// }

// // func (coll *MemCollection) Get(id string) (Item, bool, error) {
// // 	coll.mu.Lock()
// // 	defer coll.mu.Unlock()
// // 	item, ok := coll.items[id]
// // 	return item, ok, nil
// // }

// func (coll *MemCollection) Update(u Update) error {
// 	coll.mu.Lock()
// 	defer coll.mu.Unlock()
// 	if item, ok := coll.items[u.ID]; ok {
// 		switch strings.ToLower(u.Field) {
// 		case "title":
// 			item.Title = u.Value
// 		case "format":
// 			item.Format = u.Value
// 		}
// 		coll.items[u.ID] = item
// 	}
// 	return nil
// }

// func (coll *MemCollection) ForgetURL(url string) ([]string, error) {
// 	coll.mu.Lock()
// 	defer coll.mu.Unlock()
// 	deleted := []string{}
// 	for id, item := range coll.items {
// 		if url == item.URL {
// 			delete(coll.items, id)
// 			deleted = append(deleted, id)
// 		}
// 	}
// 	return deleted, nil
// }
