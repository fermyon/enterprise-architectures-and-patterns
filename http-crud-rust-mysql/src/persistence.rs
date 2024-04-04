use crate::models::Item;
use anyhow::{bail, Context, Result};

use spin_sdk::{
    mysql::{Connection, ParameterValue},
    variables,
};

const VAR_NAME_CONNECTION_STRING: &str = "db_connection_string";
const SQL_READ_ALL: &str = "SELECT Id,Name,Active FROM Items ORDER BY Name";
const SQL_READ_BY_ID: &str = "SELECT Id,Name,Active FROM Items WHERE Id=?";
const SQL_CREATE_ITEM: &str = "INSERT INTO Items (Id,Name,Active) VALUES (?,?,?)";
const SQL_UPDATE_ITEM_BY_ID: &str = "UPDATE Items SET Name=?, Active=? WHERE Id=?";
const SQL_DELETE_BY_ID: &str = "DELETE FROM Items WHERE Id=?";
const SQL_DELETE_MULTIPLE_ITEMS: &str = "DELETE FROM Items WHERE Id IN (";

fn get_connection_string() -> Result<String> {
    variables::get(VAR_NAME_CONNECTION_STRING).with_context(|| {
        format!(
            "Connection String not specified please set {}",
            VAR_NAME_CONNECTION_STRING
        )
    })
}

fn get_connection() -> Result<Connection> {
    let connection_string = get_connection_string()?;
    Connection::open(&connection_string)
        .with_context(|| "Could not establish connection to MySQL database")
}

pub(crate) fn read_all_items() -> Result<Vec<Item>> {
    let connection = get_connection()?;

    let result = connection
        .query(SQL_READ_ALL, &[])
        .with_context(|| "Error while sending query to database")?;

    result.rows.iter().map(|row| Item::try_from(row)).collect()
}

pub(crate) fn read_item_by_id(id: &str) -> Result<Option<Item>> {
    let connection = get_connection()?;

    let params = vec![ParameterValue::Str(id.to_string())];
    let result = connection
        .query(SQL_READ_BY_ID, &params)
        .with_context(|| "Error while sending query to database")?;
    let Some(row) = result.rows.first() else {
        return Ok(None);
    };
    match Item::try_from(row) {
        Ok(item) => Ok(Some(item)),
        Err(e) => Err(e),
    }
}

pub(crate) fn delete_multiple_items(ids: Vec<String>) -> Result<()> {
    if ids.is_empty() {
        bail!("Received empty list of identifiers, which is not supported");
    }
    let mut iter = ids.iter().peekable();
    let mut command = String::from(SQL_DELETE_MULTIPLE_ITEMS);
    while let Some(_) = iter.next() {
        match iter.peek() {
            Some(_) => command = format!("{}?, ", command),
            None => command = format!("{}?)", command),
        }
    }
    let connection = get_connection()?;
    let params: Vec<ParameterValue> = ids.into_iter().map(|id| ParameterValue::Str(id)).collect();
    connection
        .execute(&command, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub(crate) fn delete_item_by_id(id: &str) -> Result<()> {
    let connection = get_connection()?;

    let params = vec![ParameterValue::Str(id.to_string())];
    connection
        .execute(SQL_DELETE_BY_ID, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub(crate) fn update_item(item: &Item) -> Result<()> {
    let connection = get_connection()?;

    let params = vec![
        ParameterValue::Str(item.name.clone()),
        ParameterValue::Boolean(item.active),
        ParameterValue::Str(item.id.clone()),
    ];
    connection
        .execute(SQL_UPDATE_ITEM_BY_ID, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub(crate) fn create_item(item: &Item) -> Result<()> {
    let connection = get_connection()?;

    let params = vec![
        ParameterValue::Str(item.id.clone()),
        ParameterValue::Str(item.name.clone()),
        ParameterValue::Boolean(item.active),
    ];
    connection
        .execute(SQL_CREATE_ITEM, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}
