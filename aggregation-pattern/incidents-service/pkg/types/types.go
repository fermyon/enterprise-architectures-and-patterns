package types

type IncidentListModel struct {
	Id           string  `json:"id"`
	Amount       float64 `json:"amount"`
	CustomerName string  `json:"customerName"`
}

type IncidentDetailsModel struct {
	Id           string  `json:"id"`
	Amount       float64 `json:"amount"`
	CustomerName string  `json:"customerName"`
	Category     string  `json:"category"`
}
