use serde::{Deserialize, Serialize};

/// Request model for creating a new product
#[derive(Debug, Deserialize)]
pub struct CreateProductModel {
    /// name of the product
    pub name: String,
    /// description of the product
    pub description: String,
}

/// Request model for updating an existing product
#[derive(Debug, Deserialize)]
pub struct UpdateProductModel {
    /// name of the product
    pub name: String,
    /// description of the product
    pub description: String,
}

/// Response model returned once a new product has been created
#[derive(Debug, Serialize)]
pub struct ProductCreatedModel {
    /// the product identifier
    pub id: String,
    /// name of the product
    pub name: String,
    /// description of the product
    pub description: String,
}

/// Response model used once an existing product has been updated
#[derive(Debug, Serialize)]
pub struct ProductUpdatedModel {
    /// the product identifier
    pub id: String,
    /// name of the product
    pub name: String,
    /// description of the product
    pub description: String,
}
