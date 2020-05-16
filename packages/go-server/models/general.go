package models

// WSData ...
type WSData struct {
	Action  string      `json:"action"`
	Payload interface{} `json:"payload"`
}

// ClientData ...
type ClientData struct {
	RemoteAddr string `json:"remote_addr"`
	Name       string `json:"name"`
}
