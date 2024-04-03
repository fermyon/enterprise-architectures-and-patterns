#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_docs)]

//! This crate contains all queries of the CQRS sample

use anyhow::anyhow;
use spin_sdk::sqlite::{Connection, Value};
mod models;
pub use crate::models::{ProductDetailsModel, ProductListModel};

const DB_NAME: &str = "default";
const QUERY_ALL_COMMAND: &str = "SELECT ID, NAME FROM PRODUCTS ORDER BY NAME ASC";
const QUERY_SINGLE_COMMAND: &str = "SELECT ID, NAME, DESCRIPTION FROM PRODUCTS WHERE ID = ?";

/// The Queries struct encapsulates available queries
pub struct Queries {}

impl Queries {
    /// Query to retrieve all products
    pub fn all_products() -> anyhow::Result<Vec<ProductListModel>> {
        let con = Connection::open(DB_NAME)?;
        let query_result = con.execute(QUERY_ALL_COMMAND, &[])?;
        let products = query_result
            .rows()
            .map(|row| {
                let id = String::from(
                    row.get::<&str>("ID")
                        .ok_or_else(|| anyhow!("ID not present"))?,
                );
                let name = String::from(
                    row.get::<&str>("NAME")
                        .ok_or_else(|| anyhow!("NAME not present"))?,
                );
                anyhow::Ok(ProductListModel { id, name })
            })
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();
        Ok(products)
    }

    /// Query to retrieve a particular product using its identifier
    pub fn product_by_id(id: String) -> anyhow::Result<Option<ProductDetailsModel>> {
        let con = Connection::open(DB_NAME)?;
        let params = vec![Value::Text(id)];
        let query_result = con.execute(QUERY_SINGLE_COMMAND, &params)?;
        let product = match query_result.rows().next() {
            Some(row) => {
                let id = String::from(
                    row.get::<&str>("ID")
                        .ok_or_else(|| anyhow!("ID not present"))?,
                );
                let name = String::from(
                    row.get::<&str>("NAME")
                        .ok_or_else(|| anyhow!("NAME not present"))?,
                );
                let description = String::from(
                    row.get::<&str>("DESCRIPTION")
                        .ok_or_else(|| anyhow!("DESCRIPTION not present"))?,
                );
                Some(ProductDetailsModel {
                    id,
                    name,
                    description,
                })
            }
            None => None,
        };
        Ok(product)
    }
}
