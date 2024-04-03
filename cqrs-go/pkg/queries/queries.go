package queries

import (
	"github.com/fermyon/spin/sdk/go/v2/sqlite"
)

const (
	dbName           = "default"
	queryAllProducts = "SELECT ID, NAME FROM PRODUCTS ORDER BY NAME ASC"
	queryProductById = "SELECT ID, NAME, DESCRIPTION FROM PRODUCTS WHERE ID = ?"
)

// Response model used for a particular product, queried as part of a list
type ProductListModel struct {
	Id   string `json:"id"`
	Name string `json:"name"`
}

// Response model used for a particular product, queried using the product identifier
type ProductDetailsModel struct {
	Id          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

// Query to retrieve all products as a list
func AllProducts() ([]*ProductListModel, error) {
	products := make([]*ProductListModel, 0)

	con := sqlite.Open(dbName)
	defer con.Close()
	rows, err := con.Query(queryAllProducts)
	if err != nil {
		return nil, err
	}

	for rows.Next() {
		var product ProductListModel
		if err := rows.Scan(&product.Id, &product.Name); err != nil {
			return nil, err
		}
		products = append(products, &product)
	}
	return products, nil
}

// Query to retrieve a particular product using its identifier
func ProductById(id string) (*ProductDetailsModel, error) {
	con := sqlite.Open(dbName)
	defer con.Close()
	rows, err := con.Query(queryProductById, id)
	if err != nil {
		return nil, err
	}
	found := rows.Next()
	if !found {
		return nil, nil
	}
	var product ProductDetailsModel
	if err := rows.Scan(&product.Id, &product.Name, &product.Description); err != nil {
		return nil, err
	}
	return &product, nil
}
