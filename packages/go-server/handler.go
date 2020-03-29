package main

// Handler ...
type Handler struct {
	events map[string]func(c *Client, p interface{})
}

// NewHandler ...
func NewHandler() *Handler {
	return &Handler{
		events: make(map[string]func(c *Client, p interface{})),
	}
}

// On ...
func (h *Handler) On(an string, a func(c *Client, p interface{})) {
	h.events[an] = a
}

// InternalEmit emit internal handler functions
func (h *Handler) InternalEmit(an string, c *Client, p interface{}) {
	a, ok := h.events[an]

	if ok {
		a(c, p)
	}
}
