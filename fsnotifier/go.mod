module fsnotifier

go 1.21.5

require (
	benlittle.dev/media-manager/protocol v0.0.0
	github.com/fsnotify/fsnotify v1.7.0
	github.com/google/uuid v1.6.0
	nhooyr.io/websocket v1.8.11
)

require golang.org/x/sys v0.4.0 // indirect

replace benlittle.dev/media-manager/protocol => ../protocol
