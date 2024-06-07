package main

type MemStorage struct {
	items map[string]string
}

func NewMemStorage() (m *MemStorage) {
	m = &MemStorage{
		items: make(map[string]string),
	}
	return
}

func (m *MemStorage) List() (items map[string]string, err error) {
	items = m.items
	return
}

func (m *MemStorage) Get(id string) (item string, ok bool) {
	item, ok = m.items[id]
	return
}

func (m *MemStorage) Assign(key string, val string) {
	m.items[key] = val
	return
}

func (m *MemStorage) Delete(id string) {
	delete(m.items, id)
	return
}
