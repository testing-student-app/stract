package main

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
	ch.handler.On("test", ch.Test)
}

// Test ...
func (ch *ClienHandler) Test(c *Client, p interface{}) {
	c.send <- []byte("hi from test")
}
