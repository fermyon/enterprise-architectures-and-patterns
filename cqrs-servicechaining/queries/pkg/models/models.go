package models

type EmployeeListModel struct {
	Id   string `json:"id"`
	Name string `json:"name"`
	City string `json:"city"`
}

type EmployeeDetailsModel struct {
	Id        string              `json:"id"`
	FirstName string              `json:"firstName"`
	LastName  string              `json:"lastName"`
	Address   AddressDetailsModel `json:"address"`
}

type AddressDetailsModel struct {
	Street string `json:"street"`
	Zip    string `json:"zip"`
	City   string `json:"city"`
}

func NewEmployeeDetailsModel() EmployeeDetailsModel {
	return EmployeeDetailsModel{
		Address: AddressDetailsModel{},
	}
}
