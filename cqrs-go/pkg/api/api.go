package api

import (
	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
)

func NewApiFacade() *spinhttp.Router {
	router := spinhttp.NewRouter()

	// register queries
	router.GET("/items", queryAllProducts)
	router.GET("/items/:id", queryProductById)

	// register commands
	router.POST("/items", createProduct)
	router.PUT("/items/:id", updateProduct)
	router.DELETE("/items/:id", deleteProduct)
	return router
}
