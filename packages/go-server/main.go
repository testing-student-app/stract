package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
)

var addr = flag.String("addr", ":8080", "http service address")

func main() {
	flag.Parse()

	h := NewHandler()
	hub := newHub(h)

	go hub.run()

	ah := NewAdminHandler(h)
	ch := NewClientHndler(h)

	ah.Register()
	ch.Register()

	http.HandleFunc("/ws/a", func(w http.ResponseWriter, r *http.Request) {
		serveWs(hub, true, w, r)
	})

	http.HandleFunc("/ws/c", func(w http.ResponseWriter, r *http.Request) {
		serveWs(hub, false, w, r)
	})

	fmt.Printf("Start on %s", *addr)
	err := http.ListenAndServe(*addr, nil)
	if err != nil {
		log.Fatal("ListenAndServe: ", err)
	}
}
