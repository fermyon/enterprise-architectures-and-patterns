#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_docs)]

//! This crate contains all queries of the CQRS sample

use anyhow::anyhow;
use models::{AddressDetailsModel, EmployeeDetailsModel, EmployeeListModel};
use spin_sdk::sqlite::{Connection, Value};
mod models;

const DB_NAME: &str = "default";
const QUERY_ALL_COMMAND: &str =
    "SELECT Employees.Id, Employees.LastName || ', ' || Employees.FirstName Name, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId ORDER BY NAME ASC";
const QUERY_SINGLE_COMMAND: &str = "SELECT Employees.Id, Employees.FirstName, Employees.LastName, Addresses.Street, Addresses.Zip, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId WHERE Employees.Id = ?";

/// The Queries struct encapsulates available queries
pub struct Queries {}

impl Queries {
    /// Query to retrieve all products
    pub fn all_employees() -> anyhow::Result<Vec<EmployeeListModel>> {
        let con = Connection::open(DB_NAME)?;
        let query_result = con.execute(QUERY_ALL_COMMAND, &[])?;
        let products = query_result
            .rows()
            .map(|row| {
                let id = String::from(
                    row.get::<&str>("Employees.Id")
                        .ok_or_else(|| anyhow!("Employees.Id not present"))?,
                );
                let name = String::from(
                    row.get::<&str>("Name")
                        .ok_or_else(|| anyhow!("Name not present"))?,
                );
                let city = String::from(
                    row.get::<&str>("Addresses.City")
                        .ok_or_else(|| anyhow!("Addresses.City not present"))?,
                );
                anyhow::Ok(EmployeeListModel { id, name, city })
            })
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();
        Ok(products)
    }

    /// Query to retrieve a particular product using its identifier
    pub fn employee_by_id(id: String) -> anyhow::Result<Option<EmployeeDetailsModel>> {
        let con = Connection::open(DB_NAME)?;
        let params = vec![Value::Text(id)];
        let query_result = con.execute(QUERY_SINGLE_COMMAND, &params)?;
        let product = match query_result.rows().next() {
            Some(row) => {
                let id = String::from(
                    row.get::<&str>("Employees.Id")
                        .ok_or_else(|| anyhow!("Employees.Id not present"))?,
                );
                let first_name = String::from(
                    row.get::<&str>("Employees.FirstName")
                        .ok_or_else(|| anyhow!("Employees.FirstName not present"))?,
                );
                let last_name = String::from(
                    row.get::<&str>("Employees.LastName")
                        .ok_or_else(|| anyhow!("Employees.LastName not present"))?,
                );
                let street = String::from(
                    row.get::<&str>("Addresses.Street")
                        .ok_or_else(|| anyhow!("Addresses.Street not present"))?,
                );
                let zip = String::from(
                    row.get::<&str>("Addresses.Zip")
                        .ok_or_else(|| anyhow!("Addresses.Zip not present"))?,
                );
                let city = String::from(
                    row.get::<&str>("Addresses.City")
                        .ok_or_else(|| anyhow!("Addresses.City not present"))?,
                );
                Some(EmployeeDetailsModel {
                    id: id.clone(),
                    first_name,
                    last_name,
                    address: AddressDetailsModel {
                        id: id.clone(),
                        street,
                        zip,
                        city,
                    },
                })
            }
            None => None,
        };
        Ok(product)
    }
}
