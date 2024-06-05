package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
	"github.com/queries/pkg/persistence"
)

func init() {
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		router := spinhttp.NewRouter()
		router.GET("/employees", getAllEmployees)
		router.GET("/employees/:id", getEmployeeById)
		router.ServeHTTP(w, r)
	})
}

func getAllEmployees(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	all, err := persistence.GetAllEmployees()
	if err != nil {
		http.Error(w, fmt.Sprintf("Error loading all employees: %v", err), 500)
		return
	}
	enc := json.NewEncoder(w)
	err = enc.Encode(all)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error encoding all employees: %v", err), 500)
		return
	}
	w.Header().Set("Content-Type", "application/json")
}

func getEmployeeById(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	id := params.ByName("id")
	if len(id) == 0 {
		http.Error(w, "Bad Request", 400)
		return
	}
	found, err := persistence.GetEmployeeById(id)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error loading a specific employee: %v", err), 500)
		return
	}
	if found == nil {
		http.Error(w, "Not Found", 404)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	enc := json.NewEncoder(w)
	err = enc.Encode(found)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error encoding a specific employee: %v", err), 500)
		return
	}
}
func main() {}
