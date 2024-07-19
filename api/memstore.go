package main

type MemStore struct {
	collections map[string]Collection
}

func NewMemStore() *MemStore {
	return &MemStore{}
}

func (store *MemStore) Coll(name string) (Collection, bool, error) {
	coll, ok := store.collections[name]
	return coll, ok, nil
}
