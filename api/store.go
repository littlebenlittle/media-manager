package main

type Store interface {
	Coll(string) (Collection, bool, error)
}

type Collection struct {
	items map[string]Item
}

func (coll *Collection) List() ([]Item, error) {
	list := make([]Item, len(coll.items))
	i := 0
	for _, item := range coll.items {
		list[i] = item
		i++
	}
	return list, nil
}

func (coll *Collection) Get(id string) (Item, bool, error) {
	item, ok := coll.items[id]
	return item, ok, nil
}

func (coll *Collection) Update(id string, field string, value string) (bool, error) {
	if item, ok := coll.items[id]; ok {
		item.Meta[field] = value
		return true, nil
	} else {
		return false, nil
	}
}

type Item struct {
	ID  string
	Url string
	// If Meta contains a field "id", it will
	// be overwritten in responses to clients
	Meta map[string]string
}
