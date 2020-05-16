package main

import (
	"encoding/json"
	"fmt"

	"gitlab.com/Reidond/stract/models"
)

// AdminHandler ...
type AdminHandler struct {
	handler *Handler
}

// NewAdminHandler ...
func NewAdminHandler(handler *Handler) *AdminHandler {
	return &AdminHandler{
		handler: handler,
	}
}

// Register ...
func (ah *AdminHandler) Register() {
	ah.handler.On("clientlist", ah.ClientList)
}

// ClientList ...
func (ah *AdminHandler) ClientList(c *Client, p interface{}) {
	values := []models.ClientData{}
	for _, value := range c.hub.clients {
		values = append(values, *value)
	}

	b, err := json.Marshal(models.WSData{
		Action: "setUsers",
		Payload: values,
	})

	if err != nil {
		fmt.Printf("\nClientListError: %s", err)
	}

	if c != nil {
		c.send <- b
	}
}

// SetTests ...
func (ah *AdminHandler) SetTests(c *Client, p interface{}) {
	b, err := json.Marshal(models.WSData{
		Action: "setTests",
		Payload: "ok",
	})

	if err != nil {
		fmt.Printf("\nSetTestsError: %s", err)
	}

	if c != nil {
		c.send <- b
	}
}
