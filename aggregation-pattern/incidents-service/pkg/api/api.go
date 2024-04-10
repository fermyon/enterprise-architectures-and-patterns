package api

import (
	"encoding/json"
	"net/http"

	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
)

func New() *spinhttp.Router {
	r := spinhttp.NewRouter()
	r.GET("/incidents/items", getAllIncidents)
	r.GET("/incidents/items/:id", getIncidentById)
	r.GET("/incidents/grouped-by-customer", getIncidentsGroupedByCustomer)
	return r
}

func sendAsJson(w http.ResponseWriter, data interface{}) {
	header := w.Header()
	header.Set("Content-Type", "application/json")

	encoder := json.NewEncoder(w)
	encoder.SetIndent("", "  ")
	encoder.Encode(data)
}
