use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EmployeeListModel {
    pub id: String,
    pub name: String,
    pub city: String,
}

#[derive(Debug, Serialize)]
pub struct EmployeeDetailsModel {
    pub id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub address: AddressDetailsModel,
}

#[derive(Debug, Serialize)]
pub struct AddressDetailsModel {
    pub id: String,
    pub street: String,
    pub city: String,
    pub zip: String,
}
