package main

import "gitlab.com/Reidond/stract/models"

// Hub maintains the set of active clients and broadcasts messages to the
// clients.
type Hub struct {
	// Registered clients.
	clients map[*Client]*models.ClientData

	admin *Client

	// Inbound messages from the clients.
	broadcast chan []byte

	// Register requests from the clients.
	register chan *Client

	// Unregister requests from clients.
	unregister chan *Client

	handler *Handler
}

func newHub(h *Handler) *Hub {
	return &Hub{
		broadcast:  make(chan []byte),
		register:   make(chan *Client),
		unregister: make(chan *Client),
		clients:    make(map[*Client]*models.ClientData),
		admin:      nil,
		handler:    h,
	}
}

func (h *Hub) run() {
	for {
		select {
		case client := <-h.register:
			if !client.admin {
				h.clients[client] = &models.ClientData{
					RemoteAddr: client.conn.RemoteAddr().String(),
					Name:       "",
				}
				h.handler.InternalEmit("clientlist", h.admin, nil)
			} else {
				h.admin = client
			}
		case client := <-h.unregister:
			if _, ok := h.clients[client]; ok {
				delete(h.clients, client)
				close(client.send)
				h.handler.InternalEmit("clientlist", h.admin, nil)
			} else {
				close(client.send)
			}
		case message := <-h.broadcast:
			for client := range h.clients {
				select {
				case client.send <- message:
				default:
					close(client.send)
					delete(h.clients, client)
				}
			}
		}
	}
}
