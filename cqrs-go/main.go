package main

import (
	"net/http"

	"github.com/cqrs_go/pkg/api"
	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
)

func init() {
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		router := api.NewApiFacade()
		router.ServeHTTP(w, r)
	})
}

func main() {}
