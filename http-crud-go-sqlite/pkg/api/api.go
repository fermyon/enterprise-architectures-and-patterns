package api

import (
	"encoding/json"
	"net/http"

	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
)

func New() *spinhttp.Router {
	r := spinhttp.NewRouter()
	r.GET("/items", getAllItems)
	r.GET("/items/:id", getItemById)
	r.POST("/items", createItem)
	r.PUT("/items/:id", updateItemById)
	r.DELETE("/items/:id", deleteItemById)
	r.DELETE("/items", deleteMultipleItems)
	return r
}

func sendAsJson(w http.ResponseWriter, data interface{}) {
	header := w.Header()
	header.Set("Content-Type", "application/json")

	encoder := json.NewEncoder(w)
	encoder.SetIndent("", "  ")
	encoder.Encode(data)
}
