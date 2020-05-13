package models

// WSData ...
type WSData struct {
	Action string      `json:"action"`
	Paylod interface{} `json:"payload"`
}
