package main

import (
	"github.com/mitchellh/mapstructure"
)

// ClienHandler ...
type ClienHandler struct {
	handler *Handler
}

// NewClientHndler ...
func NewClientHndler(handler *Handler) *ClienHandler {
	return &ClienHandler{
		handler: handler,
	}
}

// Register ...
func (ch *ClienHandler) Register() {
	ch.handler.On("setname", ch.SetName)
}

// SetName ...
func (ch *ClienHandler) SetName(c *Client, p interface{}) {
	type Payload struct {
		Name string `json:"name"`
	}

	payload := Payload{}

	mapstructure.Decode(p, &payload)

	d, ok := c.hub.clients[c]

	if ok {
		d.Name = payload.Name
	}

	c.hub.handler.InternalEmit("clientlist", c.hub.admin, nil)
}
