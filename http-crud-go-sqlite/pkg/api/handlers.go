package api

import (
	"encoding/json"
	"net/http"

	"github.com/fermyon/enterprise-architectures-and-patterns/http-crud-go-sqlite/pkg/persistence"
	"github.com/fermyon/enterprise-architectures-and-patterns/http-crud-go-sqlite/pkg/types"
	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
)

func getAllItems(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	items, err := persistence.ReadAllItems(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	sendAsJson(w, items)
}

func getItemById(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	item, err := persistence.ReadItemById(r.Context(), params.ByName("id"))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if item == nil {
		http.NotFound(w, r)
		return
	}
	sendAsJson(w, item)
}

func createItem(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	var model types.ItemCreateModel
	decoder := json.NewDecoder(r.Body)
	err := decoder.Decode(&model)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	item, err := persistence.CreateItem(r.Context(), model)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	sendAsJson(w, item)

}

func updateItemById(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	var model types.ItemUpdateModel
	decoder := json.NewDecoder(r.Body)
	err := decoder.Decode(&model)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	item, err := persistence.UpdateItemById(r.Context(), params.ByName("id"), model)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if item == nil {
		http.NotFound(w, r)
		return
	}
	sendAsJson(w, item)
}

func deleteMultipleItems(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	var model types.BatchDeleteModel
	decoder := json.NewDecoder(r.Body)
	err := decoder.Decode(&model)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	err = persistence.DeleteMultipleItems(r.Context(), model)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

func deleteItemById(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	err := persistence.DeleteItemById(r.Context(), params.ByName("id"))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}
