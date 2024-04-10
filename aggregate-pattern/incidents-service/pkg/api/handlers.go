package api

import (
	"net/http"

	"github.com/fermyon/enterprise-architectures-and-patterns/aggregate-pattern/incidents_service/pkg/types"
	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
	"github.com/fermyon/spin/sdk/go/v2/sqlite"
)

const (
	dbName              = "default"
	sqlReadAllIncidents = "SELECT Id, Amount, CustomerName FROM Incidents"
	sqlReadIncidentById = "SELECT Id, Amount, CustomerName, Category FROM Incidents where Id=?"
)

func getAllIncidents(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	con := sqlite.Open(dbName)
	defer con.Close()
	rows, err := con.Query(sqlReadAllIncidents)
	if err != nil {
		http.Error(w, "Error while loading incidents from database", 500)
		return
	}
	defer rows.Close()
	var incidents []*types.IncidentListModel
	for rows.Next() {
		var incident types.IncidentListModel
		if err := rows.Scan(&incident.Id, &incident.Amount, &incident.CustomerName); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		incidents = append(incidents, &incident)
	}
	sendAsJson(w, incidents)

}

func getIncidentById(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	id := params.ByName("id")
	if len(id) == 0 {
		http.Error(w, "Bad Request", 400)
		return
	}
	con := sqlite.Open(dbName)
	defer con.Close()
	rows, err := con.Query(sqlReadIncidentById, id)
	if err != nil {
		http.Error(w, "Error while loading incidents from database", 500)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var item types.IncidentDetailsModel
		err = rows.Scan(&item.Id, &item.Amount, &item.CustomerName, &item.Category)
		if err != nil {
			http.Error(w, "Error materializing incident", 500)
			return
		}

		sendAsJson(w, item)
		return
	}
	http.Error(w, "", 404)

}

type Pair struct {
	Key   string
	Value float64
}

func getIncidentsGroupedByCustomer(w http.ResponseWriter, r *http.Request, params spinhttp.Params) {
	con := sqlite.Open(dbName)
	defer con.Close()
	rows, err := con.Query(sqlReadAllIncidents)
	if err != nil {
		http.Error(w, "Error while loading most expensive incidents from database", 500)
		return
	}
	defer rows.Close()
	var incidents []*types.IncidentListModel
	for rows.Next() {
		var incident types.IncidentListModel
		if err := rows.Scan(&incident.Id, &incident.Amount, &incident.CustomerName); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		incidents = append(incidents, &incident)
	}
	groupedByCustomer := make(map[string][]*types.IncidentListModel, 0)
	for _, val := range incidents {
		i, ok := groupedByCustomer[val.CustomerName]
		if !ok {
			i = make([]*types.IncidentListModel, 0)
		}
		i = append(i, val)
		groupedByCustomer[val.CustomerName] = i
	}

	sendAsJson(w, groupedByCustomer)

}
