#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_docs)]

//! This crate contains all commands of the CQRS sample

mod models;
pub use crate::models::{
    CreateProductModel, ProductCreatedModel, ProductUpdatedModel, UpdateProductModel,
};
use spin_sdk::sqlite::{Connection, Value};
use uuid::Uuid;

const DB_NAME: &str = "cqrs";
const COMMAND_CREATE_PRODUCT: &str = "INSERT INTO PRODUCTS (ID, NAME, DESCRIPTION) VALUES (?,?,?)";
const COMMAND_UPDATE_PRODUCT: &str =
    "UPDATE PRODUCTS SET NAME = ?, DESCRIPTION = ? WHERE ID = ? RETURNING ID";
const COMMAND_DELETE_PRODUCT: &str = "DELETE FROM PRODUCTS WHERE ID = ? RETURNING ID";

/// The Queries struct encapsulates available commands
pub struct Commands {}

impl Commands {
    /// Command to create a new product and store it in the database
    pub fn create_product(model: CreateProductModel) -> anyhow::Result<ProductCreatedModel> {
        let id = Uuid::new_v4();
        let con = Connection::open(DB_NAME)?;
        let params = [
            Value::Text(id.to_string()),
            Value::Text(model.name.clone()),
            Value::Text(model.description.clone()),
        ];
        let _ = con.execute(COMMAND_CREATE_PRODUCT, &params)?;
        Ok(ProductCreatedModel {
            id: id.to_string(),
            name: model.name,
            description: model.description,
        })
    }

    /// Command to update an existing product in the datastore
    pub fn update_product(
        id: String,
        model: UpdateProductModel,
    ) -> anyhow::Result<Option<ProductUpdatedModel>> {
        let con = Connection::open(DB_NAME)?;
        let params = [
            Value::Text(model.name.clone()),
            Value::Text(model.description.clone()),
            Value::Text(id.clone()),
        ];
        let query_result = con.execute(COMMAND_UPDATE_PRODUCT, &params)?;
        let affected = query_result.rows.into_iter().count();
        if affected < 1 {
            return Ok(None);
        }
        Ok(Some(ProductUpdatedModel {
            id: id,
            name: model.name,
            description: model.description,
        }))
    }

    /// Command delete a particular product from the datastore using its identifier
    pub fn delete_product_by_id(id: String) -> anyhow::Result<bool> {
        let con = Connection::open(DB_NAME)?;
        let params = [Value::Text(id.clone())];
        let query_result = con.execute(COMMAND_DELETE_PRODUCT, &params)?;
        let count = query_result.rows().into_iter().count();
        Ok(count > 0)
    }

    /// ensure database exists
    pub fn ensure_database() -> anyhow::Result<()> {
        let con = Connection::open(DB_NAME)?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS PRODUCTS ( ID VARCHAR(36) PRIMARY KEY, NAME TEXT NOT NULL, DESCRIPTION TEXT NOT NULL)",
            &[],
        )?;
        Ok(())
    }
}
