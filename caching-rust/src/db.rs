use anyhow::Context;
use spin_sdk::sqlite::{Connection, Value};

use crate::model::Item;

pub(crate) fn get_all_from_database() -> anyhow::Result<Vec<Item>> {
    let con = Connection::open_default()?;
    let query_result = con.execute("SELECT ID, NAME FROM ITEMS", &[])?;
    let items: Vec<Item> = query_result
        .rows
        .iter()
        .map(|row| Item {
            id: row.get::<&str>(0).unwrap_or_default().to_string(),
            name: row.get::<&str>(1).unwrap_or_default().to_string(),
        })
        .collect();
    Ok(items)
}

pub(crate) fn get_single_from_database(id: String) -> anyhow::Result<Option<Item>> {
    let con = Connection::open_default()?;
    let params = [Value::Text(id.clone())];
    let query_result = con.execute("SELECT NAME FROM ITEMS WHERE ID = ?", &params)?;
    let res = match query_result.rows().next() {
        None => None,
        Some(row) => Some(Item {
            id: id,
            name: row.get::<&str>("NAME").unwrap_or_default().to_string(),
        }),
    };
    Ok(res)
}

pub(crate) fn update_single_item_in_database(id: String, name: String) -> anyhow::Result<Item> {
    let con = Connection::open_default()?;
    let params = [Value::Text(name.clone()), Value::Text(id.clone())];
    con.execute("UPDATE ITEMS SET NAME=? WHERE ID = ?", &params)
        .and(Ok(Item { id, name }))
        .with_context(|| "Error while updating item in database")
}
