use anyhow::Result;
use spin_sdk::sqlite::{Connection, Value};
use uuid::Uuid;

use crate::models::{
    AddressCreatedModel, AddressUpdatedModel, CreateEmployeeModel, EmployeeCreatedModel,
    EmployeeUpdatedModel, UpdateEmployeeModel,
};

const COMMAND_CREATE_EMPLOYEE: &str =
    "INSERT INTO Employees (Id, FirstName, LastName) VALUES (?,?,?);";
const COMMAND_CREATE_ADDRESS: &str =
    "INSERT INTO Addresses (EmployeeId, Street, Zip, City) VALUES (?,?,?,?);";

const COMMAND_UPDATE_EMPLOYEE: &str =
    "UPDATE Employees SET FirstName = ?, LastName = ? WHERE Id = ?; RETURNING Id;";
const COMMAND_UPDATE_ADDRESS: &str =
    "UPDATE Addresses SET Street = ?, Zip = ?, City = ? WHERE EmployeeId =?; RETURNING Id;";

const COMMAND_DELETE_EMPLOYEE: &str = "DELETE FROM Employees WHERE Id = ? RETURNING Id";

pub(crate) fn create_employee(model: CreateEmployeeModel) -> Result<EmployeeCreatedModel> {
    let con = Connection::open_default()?;
    let id = Uuid::new_v4();
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

pub(crate) fn delete_employee_by_id(id: &str) -> Result<bool> {
    let con = Connection::open_default()?;
    let params = [Value::Text(id.to_string())];
    let query_result = con.execute(COMMAND_DELETE_EMPLOYEE, &params)?;
    let count = query_result.rows().count();
    Ok(count > 0)
}

pub(crate) fn update_employee_by_id(
    id: &str,
    model: UpdateEmployeeModel,
) -> Result<Option<EmployeeUpdatedModel>> {
    let con = Connection::open_default()?;
    let employee_params = [
        Value::Text(model.first_name.clone()),
        Value::Text(model.last_name.clone()),
        Value::Text(id.to_string()),
    ];
    let address_params = [
        Value::Text(model.address.street.clone()),
        Value::Text(model.address.zip.clone()),
        Value::Text(model.address.city.clone()),
        Value::Text(id.to_string()),
    ];
    let _ = con.execute("BEGIN TRANSACTION;", &[]);
    let _ = con.execute(COMMAND_UPDATE_EMPLOYEE, &employee_params)?;
    let _ = con.execute(COMMAND_UPDATE_ADDRESS, &address_params)?;
    let _ = con.execute("END TRANSACTION;", &[])?;
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
