package main

// type Store interface {
// 	List() (list map[string]Metadata, err error)
// 	Get(id string) (media Media, ok bool, err error)
// 	Create(path string, meta Metadata) (id string, err error)
// 	Update(id string, fields map[string]string) (err error)
// 	Delete(id string) (err error)
// }

// type SqliteStore struct {
// 	db  *gorm.DB
// 	mut sync.Mutex
// }

// func NewSqliteStore(dir string) (store *SqliteStore, err error) {
// 	store = &SqliteStore{}
// 	store.db, err = gorm.Open(sqlite.Open(path.Join(dir, "_meta.sqlite")), &gorm.Config{})
// 	store.db.AutoMigrate(&Media{})
// 	return
// }

// func (store *SqliteStore) List() (meta map[string]Metadata, err error) {
// 	store.mut.Lock()
// 	defer store.mut.Unlock()
// 	var media []Media
// 	err = store.db.Find(&media).Error
// 	if err != nil {
// 		return
// 	}
// 	meta = make(map[string]Metadata, len(media))
// 	for _, m := range media {
// 		meta[m.ID] = m.Metadata
// 	}
// 	return
// }

// func (store *SqliteStore) Get(id string) (media Media, ok bool, err error) {
// 	store.mut.Lock()
// 	defer store.mut.Unlock()
// 	result := store.db.First(&media, id)
// 	ok = result.RowsAffected == 0
// 	err = result.Error
// 	return
// }

// func (store *SqliteStore) Create(path string, meta Metadata) (id string, err error) {
// 	store.mut.Lock()
// 	defer store.mut.Unlock()
// 	err = store.db.Model(&Media{}).
// 		Select("id").
// 		Where("path = ?", path).
// 		Take(&id).
// 		Error
// 	if err != nil {
// 		return
// 	}
// 	if id != "" {
// 		err = fmt.Errorf("path already registered in database with id `%s`", id)
// 		return
// 	}
// 	media := Media{
// 		Metadata: meta,
// 		Path:     path,
// 		ID:       uuid.New().String(),
// 	}
// 	err = store.db.Create(&media).Error
// 	id = media.ID
// 	return
// }

// func (store *SqliteStore) Update(id string, fields map[string]string) (err error) {
// 	f := make(map[string]any, len(fields))
// 	for k, v := range fields {
// 		f[k] = v
// 	}
// 	err = store.db.Model(&Media{}).
// 		Where("id = ?", id).
// 		Updates(fields).
// 		Error
// 	return
// }

// func (store *SqliteStore) Delete(id string) (err error) {
// 	store.mut.Lock()
// 	defer store.mut.Unlock()
// 	err = store.db.Delete(&Media{ID: id}).Error
// 	return
// }
