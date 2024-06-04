#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_docs)]

//! This crate contains all commands of the CQRS sample

mod models;
pub use models::{
    AddressCreatedModel, AddressUpdatedModel, CreateEmployeeModel, EmployeeCreatedModel,
    EmployeeUpdatedModel, UpdateEmployeeModel,
};
use spin_sdk::sqlite::{Connection, Value};
use uuid::Uuid;

const DB_NAME: &str = "default";
const COMMAND_CREATE_EMPLOYEE: &str =
    "INSERT INTO Employees (Id, FirstName, LastName) VALUES (?,?,?);";
const COMMAND_CREATE_ADDRESS: &str =
    "INSERT INTO Addresses (EmployeeId, Street, Zip, City) VALUES (?,?,?,?);";

const COMMAND_UPDATE_EMPLOYEE: &str =
    "UPDATE Employees SET FirstName = ?, LastName = ? WHERE Id = ?; UPDATE Addresses SET Street = ?, Zip = ?, City = ? WHERE EmployeeId =?; RETURNING ID";

const COMMAND_DELETE_EMPLOYEE: &str = "DELETE FROM Employees WHERE Id = ? RETURNING Id";

/// The Queries struct encapsulates available commands
pub struct Commands {}

impl Commands {
    /// Command to create a new employee
    pub fn create_employee(model: CreateEmployeeModel) -> anyhow::Result<EmployeeCreatedModel> {
        let id = Uuid::new_v4();
        let con = Connection::open(DB_NAME)?;
        let employee_params = [
            Value::Text(id.to_string()),
            Value::Text(model.first_name.clone()),
            Value::Text(model.last_name.clone()),
        ];
        let address_params = [
            Value::Text(id.to_string()),
            Value::Text(model.address.street.clone()),
            Value::Text(model.address.zip.clone()),
            Value::Text(model.address.city.clone()),
        ];
        let _ = con.execute("BEGIN TRANSACTION;", &[]);
        let _ = con.execute(COMMAND_CREATE_EMPLOYEE, &employee_params)?;
        let _ = con.execute(COMMAND_CREATE_ADDRESS, &address_params);
        let _ = con.execute("END TRANSACTION;", &[]);
        Ok(EmployeeCreatedModel {
            id: id.to_string(),
            first_name: model.first_name,
            last_name: model.last_name,
            address: AddressCreatedModel {
                id: id.to_string(),
                street: model.address.street,
                zip: model.address.zip,
                city: model.address.city,
            },
        })
    }

    /// Command to update an existing product in the datastore
    pub fn update_employee(
        id: String,
        model: UpdateEmployeeModel,
    ) -> anyhow::Result<Option<EmployeeUpdatedModel>> {
        let con = Connection::open(DB_NAME)?;
        let params = [
            Value::Text(model.first_name.clone()),
            Value::Text(model.last_name.clone()),
            Value::Text(id.clone()),
            Value::Text(model.address.street.clone()),
            Value::Text(model.address.zip.clone()),
            Value::Text(model.address.city.clone()),
            Value::Text(id.clone()),
        ];
        let _ = con.execute("BEGIN TRANSACTION;", &[]);
        let query_result = con.execute(COMMAND_UPDATE_EMPLOYEE, &params)?;
        let _ = con.execute("END TRANSACTION;", &[]);
        let affected = query_result.rows.len();
        if affected < 1 {
            return Ok(None);
        }
        Ok(Some(EmployeeUpdatedModel {
            id: id.to_string(),
            first_name: model.first_name,
            last_name: model.last_name,
            address: AddressUpdatedModel {
                id: id.to_string(),
                street: model.address.street,
                zip: model.address.zip,
                city: model.address.city,
            },
        }))
    }

    /// Command delete a particular product from the datastore using its identifier
    pub fn delete_employee_by_id(id: String) -> anyhow::Result<bool> {
        let con = Connection::open(DB_NAME)?;
        let params = [Value::Text(id.clone())];
        let query_result = con.execute(COMMAND_DELETE_EMPLOYEE, &params)?;
        let count = query_result.rows().count();
        Ok(count > 0)
    }
}
