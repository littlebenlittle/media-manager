package main

import (
	"context"
	"log"
	"net/http"

	"nhooyr.io/websocket"
	"nhooyr.io/websocket/wsjson"
)

type InMsg struct {
	Sync   *struct{}
	Forget *Forget
	Exists *Exists
}

type Exists struct {
	Data     string
	Metadata []string
}

type Forget struct {
	Data string
}

type OutMsg struct {
	Exists *Exists
	Forget *Forget
}

type Client struct {
	In  chan InMsg
	Out chan OutMsg
}

func main() {
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	db := make(map[string][]string)
	clients := make(map[chan OutMsg]struct{})
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		log.Printf("new client: %s", r.RemoteAddr)
		c, err := websocket.Accept(w, r, nil)
		if err != nil {
			log.Fatalf("%s", err)
		}
		defer c.CloseNow()
		ctx, cancel := context.WithCancel(r.Context())
		defer cancel()
		in, out := ws_to_evio(ctx, c)
		defer close(out)
		for ev := range in {
			if ev.Sync != nil {
				for d, m := range db {
					out <- OutMsg{Exists: &Exists{Data: d, Metadata: m}}
				}
			} else if ev.Forget != nil {
				if _, ok := db[ev.Forget.Data]; ok {
					delete(db, ev.Forget.Data)
					for client := range clients {
						client <- OutMsg{Forget: &Forget{Data: ev.Forget.Data}}
					}
				}
			} else if ev.Exists != nil {
				if ev.Exists.Metadata != nil {
					db[ev.Exists.Data] = ev.Exists.Metadata
				} else {
					db[ev.Exists.Data] = make([]string, 0)
				}
				for client := range clients {
					client <- OutMsg{Exists: &Exists{Data: ev.Exists.Data, Metadata: ev.Exists.Metadata}}
				}
			}
		}
	})
	addr := "0.0.0.0:80"
	log.Printf("listen: %s", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}

func ws_to_evio(ctx context.Context, c *websocket.Conn) (chan InMsg, chan OutMsg) {
	in := make(chan InMsg)
	out := make(chan OutMsg)
	go func() {
		var ev InMsg
		for {
			if err := wsjson.Read(ctx, c, &ev); err == nil {
				in <- ev
			} else {
				log.Printf("%s", err)
				close(in)
				close(out)
				break
			}
		}
	}()
	go func() {
		for r := range out {
			if err := wsjson.Write(ctx, c, r); err != nil {
				log.Printf("%s", err)
			}
		}
	}()
	return in, out
}
