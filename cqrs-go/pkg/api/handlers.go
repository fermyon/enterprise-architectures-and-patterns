package api

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"

	"github.com/cqrs_go/pkg/commands"
	"github.com/cqrs_go/pkg/queries"
	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
	"github.com/google/uuid"
)

func queryAllProducts(w http.ResponseWriter, r *http.Request, _ spinhttp.Params) {
	products, err := queries.AllProducts()
	if err != nil {
		w.WriteHeader(500)
		return
	}
	w.WriteHeader(200)
	w.Header().Add("Content-Type", "application/json")
	err = json.NewEncoder(w).Encode(products)
	if err != nil {
		// remove the Content-Type header (because no body is sent for 500)
		w.Header().Del("Content-Type")
		w.WriteHeader(500)
		return
	}
}

func queryProductById(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	id := params.ByName("id")
	if len(id) == 0 || uuid.Validate(id) != nil {
		w.WriteHeader(400)
		return
	}

	product, err := queries.ProductById(id)
	if err != nil {
		w.WriteHeader(500)
		return
	}

	// if product and err are nil, we could not find the product
	if product == nil {
		w.WriteHeader(404)
		return
	}
	w.WriteHeader(200)
	w.Header().Add("Content-Type", "application/json")
	err = json.NewEncoder(w).Encode(product)
	if err != nil {
		// remove the Content-Type header (because no body is sent for 500)
		w.Header().Del("Content-Type")
		w.WriteHeader(500)
		return
	}
}

func createProduct(w http.ResponseWriter, r *http.Request, _ spinhttp.Params) {
	var model commands.CreateProductModel
	err := json.NewDecoder(r.Body).Decode(&model)
	if err != nil {
		w.WriteHeader(400)
		return
	}

	p, err := commands.CreateProduct(model)
	if err != nil {
		w.WriteHeader(500)
		return
	}
	loc := buildLocationHeader(r.URL.String(), p.Id)
	w.WriteHeader(200)
	w.Header().Add("Content-Type", "application/json")
	w.Header().Add("Location", loc)
	err = json.NewEncoder(w).Encode(p)
	if err != nil {
		// remove the Content-Type header (because no body is sent for 500)
		w.Header().Del("Content-Type")
		// remove the Location header
		w.Header().Del("Location")
		w.WriteHeader(500)
		return
	}
}

func updateProduct(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	id := params.ByName("id")
	if len(id) == 0 || uuid.Validate(id) != nil {
		w.WriteHeader(400)
		return
	}
	var model commands.UpdateProductModel
	err := json.NewDecoder(r.Body).Decode(&model)
	if err != nil {
		w.WriteHeader(400)
		return
	}

	product, err := commands.UpdateProduct(id, model)
	if err != nil {
		w.WriteHeader(500)
		return
	}
	// if err and product are nil, we were not able to find the product
	if product == nil {
		w.WriteHeader(404)
		return
	}
	w.WriteHeader(200)
	w.Header().Add("Content-Type", "application/json")
	err = json.NewEncoder(w).Encode(product)
	if err != nil {
		w.WriteHeader(500)
		return
	}
}

func deleteProduct(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	id := params.ByName("id")
	if len(id) == 0 || uuid.Validate(id) != nil {
		w.WriteHeader(400)
		return
	}

	found, err := commands.DeleteProduct(id)
	if err != nil {
		w.WriteHeader(500)
		return
	}
	// found == false indicates that we were not able to find the particular product identifier
	if !found {
		w.WriteHeader(404)
		return
	}
	w.WriteHeader(204)
}

func buildLocationHeader(requestUrl string, id string) string {
	if strings.HasSuffix(requestUrl, "/") {
		return fmt.Sprintf("%s%s", requestUrl, id)
	}
	return fmt.Sprintf("%s/%s", requestUrl, id)
}
