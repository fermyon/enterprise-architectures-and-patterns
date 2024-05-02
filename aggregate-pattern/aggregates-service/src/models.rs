use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;

#[derive(Serialize)]
pub(crate) struct DashboardModel {
    #[serde(rename = "customerCount")]
    pub customer_count: i64,
    #[serde(rename = "topCustomers")]
    pub top_5_customers: Vec<CustomerScoringModel>,
    #[serde(rename = "customersWithMostIncidents")]
    pub customers_with_most_incidents: Vec<CustomerWithNumberOfIncidentModel>,
    #[serde(rename = "customersWithHighestIncidentVolume")]
    pub customers_with_highest_incident_volume: Vec<CustomerIncidentVolumeModel>,
}

impl DashboardModel {
    pub fn from(
        customer_count: CustomerCountResponseModel,
        top_customers: Vec<CustomerListModel>,
        incidents_grouped_by_customer: IncidentsGroupedByCustomerResponseModel,
    ) -> Self {
        let top_customers: Vec<CustomerScoringModel> = top_customers
            .into_iter()
            .take(5)
            .map(|c| CustomerScoringModel {
                customer_name: c.name,
                scoring: c.scoring,
            })
            .collect();

        DashboardModel {
            customer_count: customer_count.into(),
            top_5_customers: top_customers,
            customers_with_most_incidents: (&incidents_grouped_by_customer).into(),
            customers_with_highest_incident_volume: (&incidents_grouped_by_customer).into(),
        }
    }
}
impl IntoBody for DashboardModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl From<CustomerCountResponseModel> for i64 {
    fn from(value: CustomerCountResponseModel) -> Self {
        value.count
    }
}

#[derive(Serialize)]
pub(crate) struct CustomerScoringModel {
    #[serde(rename = "customerName")]
    pub customer_name: String,
    pub scoring: u64,
}

#[derive(Serialize)]
pub(crate) struct CustomerWithNumberOfIncidentModel {
    #[serde(rename = "customerName")]
    pub customer_name: String,
    #[serde(rename = "numberOfIncidents")]
    pub number_of_incidents: u8,
}

impl From<&IncidentsGroupedByCustomerResponseModel> for Vec<CustomerWithNumberOfIncidentModel> {
    fn from(value: &IncidentsGroupedByCustomerResponseModel) -> Self {
        let mut incidents_count: HashMap<&String, usize> = HashMap::new();

        // Iterate over the customers and count incidents
        for (customer, incidents) in &value.customers {
            *incidents_count.entry(customer).or_insert(0) += incidents.len();
        }

        // Sort the customers by the number of incidents
        let mut sorted_customers: Vec<CustomerWithNumberOfIncidentModel> = incidents_count
            .into_iter()
            .map(|ic| CustomerWithNumberOfIncidentModel {
                customer_name: ic.0.to_string(),
                number_of_incidents: ic.1 as u8,
            })
            .collect();
        sorted_customers.sort_by(|a, b| b.number_of_incidents.cmp(&a.number_of_incidents)); // Sort in descending order

        sorted_customers.into_iter().take(5).collect()
    }
}
#[derive(Serialize)]
pub(crate) struct CustomerIncidentVolumeModel {
    #[serde(rename = "customerName")]
    pub customer_name: String,
    #[serde(rename = "incidentVolume")]
    pub incident_volume: f64,
}

impl From<&IncidentsGroupedByCustomerResponseModel> for Vec<CustomerIncidentVolumeModel> {
    fn from(value: &IncidentsGroupedByCustomerResponseModel) -> Self {
        let mut incident_volume: HashMap<&String, f64> = HashMap::new();
        // Iterate over the customers and count incidents
        for (customer, incidents) in &value.customers {
            for incident in incidents {
                *incident_volume.entry(customer).or_insert(0.0) += incident.amount;
            }
        }
        let mut sorted_volumes: Vec<CustomerIncidentVolumeModel> = incident_volume
            .into_iter()
            .map(|v| CustomerIncidentVolumeModel {
                customer_name: v.0.to_string(),
                incident_volume: v.1,
            })
            .collect();
        sorted_volumes.sort_by(|a, b| b.incident_volume.partial_cmp(&a.incident_volume).unwrap());
        sorted_volumes.into_iter().take(5).collect()
    }
}

#[derive(Deserialize)]
pub struct IncidentsGroupedByCustomerResponseModel {
    #[serde(flatten)]
    pub customers: HashMap<String, Vec<IncidentResponseModel>>,
}

#[derive(Deserialize)]
pub struct CustomerCountResponseModel {
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerListModel {
    pub id: String,
    pub name: String,
    pub country: String,
    pub scoring: u64,
}

#[derive(Deserialize)]
pub struct IncidentResponseModel {
    pub id: String,
    pub amount: f64,
    #[serde(rename = "customerName")]
    pub customer_name: String,
}
