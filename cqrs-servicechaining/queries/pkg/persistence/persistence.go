package persistence

import (
	"github.com/fermyon/spin/sdk/go/v2/sqlite"
	"github.com/queries/pkg/models"
)

const (
	db                = "default"
	queryAllEmployees = "SELECT Employees.Id, Employees.LastName || ', ' || Employees.FirstName Name, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId ORDER BY NAME ASC"
	queryEmployeeById = "SELECT Employees.Id, Employees.FirstName, Employees.LastName, Addresses.Street, Addresses.Zip, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId WHERE Employees.Id = ?"
)

func GetAllEmployees() ([]models.EmployeeListModel, error) {
	con := sqlite.Open(db)
	defer con.Close()

	rows, err := con.Query(queryAllEmployees)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	all := []models.EmployeeListModel{}
	for rows.Next() {
		var e models.EmployeeListModel
		err = rows.Scan(&e.Id, &e.Name, &e.City)
		if err != nil {
			return nil, err
		}
		all = append(all, e)
	}
	return all, nil
}

func GetEmployeeById(id string) (*models.EmployeeDetailsModel, error) {
	con := sqlite.Open(db)
	defer con.Close()

	rows, err := con.Query(queryEmployeeById, id)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	for rows.Next() {
		e := models.NewEmployeeDetailsModel()
		err = rows.Scan(&e.Id, &e.FirstName, &e.LastName, &e.Address.Street, &e.Address.Zip, &e.Address.City)
		if err != nil {
			return nil, err
		}

		return &e, nil
	}

	return nil, nil
}
