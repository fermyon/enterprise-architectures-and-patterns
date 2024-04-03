package types

type Item struct {
	Id     string `json:"id" db:"ID"`
	Name   string `json:"name" db:"NAME"`
	Active bool   `json:"active" db:"ACTIVE"`
}

type ItemCreateModel struct {
	Name   string `json:"name"`
	Active bool   `json:"active"`
}

func (model *ItemCreateModel) NewItemWithId(id string) Item {
	return Item{
		Id:     id,
		Name:   model.Name,
		Active: model.Active,
	}
}

type ItemUpdateModel struct {
	Name   string `json:"name"`
	Active bool   `json:"active"`
}

type BatchDeleteModel struct {
	Ids []string `json:"ids"`
}

func (model *ItemUpdateModel) AsItem(id string) Item {
	return Item{
		Id:     id,
		Name:   model.Name,
		Active: model.Active,
	}
}
