package commands

import (
	"github.com/fermyon/spin/sdk/go/v2/sqlite"
	"github.com/google/uuid"
)

const dbName string = "default"

// Request model for creating new products
type CreateProductModel struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

// Request model for updating a particular product
type UpdateProductModel struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

// Response model used once a product has been created
type ProductCreatedModel struct {
	Id          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

// Response model used once a product has been updated
type ProductUpdatedModel struct {
	Id          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

const (
	commandCreateProduct = "INSERT INTO PRODUCTS (ID, NAME, DESCRIPTION) VALUES (?,?,?)"
	commandUpdateProduct = "UPDATE PRODUCTS SET NAME = ?, DESCRIPTION = ? WHERE ID = ? RETURNING ID"
	commandDeleteProduct = "DELETE FROM PRODUCTS WHERE ID = ? RETURNING ID"
)

// Command to create a new product
func CreateProduct(model CreateProductModel) (*ProductCreatedModel, error) {
	con := sqlite.Open(dbName)
	defer con.Close()
	id, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}

	_, err = con.Exec(commandCreateProduct, id.String(), model.Name, model.Description)
	if err != nil {
		return nil, err
	}
	return &ProductCreatedModel{
		Id:          id.String(),
		Name:        model.Name,
		Description: model.Description,
	}, nil
}

// Command to update a particular product
func UpdateProduct(id string, model UpdateProductModel) (*ProductUpdatedModel, error) {
	con := sqlite.Open(dbName)
	defer con.Close()
	res, err := con.Query(commandUpdateProduct, model.Name, model.Description, id)
	if err != nil {
		return nil, err
	}
	updated := res.Next()
	if !updated {
		return nil, nil
	}

	return &ProductUpdatedModel{
		Id:          id,
		Name:        model.Name,
		Description: model.Description,
	}, nil
}

// Command to delete a particular product
func DeleteProduct(id string) (bool, error) {
	con := sqlite.Open(dbName)
	defer con.Close()
	res, err := con.Query(commandDeleteProduct, id)
	if err != nil {
		return false, err
	}
	updated := res.Next()
	if !updated {
		return false, nil
	}
	return true, nil
}
