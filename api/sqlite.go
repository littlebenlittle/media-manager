package main

// import (
// 	"database/sql"
// 	"fmt"
// 	"log"
// 	"slices"
// 	"strings"

// 	"github.com/google/uuid"
// 	_ "github.com/mattn/go-sqlite3"
// )

// type SqliteStorage struct {
// 	db *sql.DB
// }

// func NewSqliteStorage() (s *SqliteStorage, err error) {
// 	s = &SqliteStorage{}
// 	s.db, err = sql.Open("sqlite3", "/metadata/metadata.sqlite")
// 	if err != nil {
// 		return
// 	}
// 	_, err = s.db.Exec(`
// 		CREATE TABLE IF NOT EXISTS media (
// 			id   STRING PRIMARY KEY,
// 			path STRING NOT NULL
// 		);

// 		CREATE TABLE IF NOT EXISTS metadata (
// 			id        STRING PRIMARY KEY,
// 			media_id  STRING NOT NULL UNIQUE,
// 			title     STRING,
// 			shortname STRING,
// 			format    STRING,

// 			CONSTRAINT fk_media_id
// 				FOREIGN KEY (media_id) REFERENCES  media(id)
// 		);
// 	`)
// 	return
// }

// func (m *SqliteStorage) List() (media []Media, ok bool) {
// 	rows, err := m.db.Query(`
// 	    SELECT (
// 			media.id,
// 			media.path,
// 			metadata.title,
// 			metadata.shortname,
// 			metadata.format
// 		)
// 		FROM media LEFT JOIN metadata ON media.id = metadata.media_id;
// 	`)
// 	if err != nil {
// 		log.Printf("%s", err)
// 		return
// 	}
// 	media = []Media{}
// 	for rows.Next() {
// 		var m Media
// 		if err = rows.Scan(
// 			&m.ID,
// 			&m.Path,
// 			&m.Metadata.Title,
// 			&m.Metadata.Shortname,
// 			&m.Metadata.Format,
// 		); err != nil {
// 			return
// 		}
// 		media = append(media, m)
// 	}
// 	return
// }

// func (m *SqliteStorage) Get(id string) (media Media, ok bool, err error) {
// 	row := m.db.QueryRow(`
// 	    SELECT (
// 			media.id,
// 			media.path,
// 			metadata.title,
// 			metadata.shortname,
// 			metadata.format
// 		)
// 		FROM media LEFT JOIN metata ON media.id = metadata.media_id
// 		WHERE media.id = ?;
// 	`, id)
// 	err = row.Scan(
// 		&media.ID,
// 		&media.Path,
// 		&media.Metadata.Title,
// 		&media.Metadata.Shortname,
// 		&media.Metadata.Format,
// 	)
// 	return
// }

// func (m *SqliteStorage) Put(path string, metadata Media) (id string, err error) {
// 	id = uuid.New().String()
// 	_, err = m.db.Exec(`
// 		INSERT INTO media (id, path)
// 		VALUES (?, ?);
// 	`, id, path)
// 	if err != nil {
// 		return
// 	}
// 	_, err = m.db.Exec(`
// 		INSERT INTO metadata (title, shortname, format)
// 		VALUES (?, ?, ?);
// 	`, metadata.Title, metadata.Shortname, metadata.Format)
// 	return
// }

// func (m *SqliteStorage) Drop(id string) (err error) {
// 	_, err = m.db.Exec(`
// 		DELETE FROM media
// 		WHERE id = ?
// 	`, id)
// 	return
// }

// var allowed_fields = []string{"title", "shortname", "format"}

// func (m *SqliteStorage) Update(id string, fields map[string]string) (err error) {
// 	updates := []string{}
// 	values := []any{}
// 	for f, v := range fields {
// 		if slices.Contains(allowed_fields, f) {
// 			updates = append(updates, fmt.Sprintf("%s = ?", f))
// 			values = append(values, v)
// 		}
// 	}
// 	if len(updates) == 0 {
// 		return
// 	}
// 	update_string := strings.Join(updates, ", ")
// 	values = append(values, id)
// 	_, err = m.db.Exec(`
// 		UPDATE metadata
// 		SET `+update_string+`
// 		WHERE media.id = ?;
// 	`, values...)
// 	return
// }

// func (m *SqliteStorage) Close() {
// 	m.db.Close()
// }
