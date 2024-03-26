use serde::Serialize;

/// ProductListModel defines the layout of a product when queried as list
#[derive(Debug, Serialize)]
pub struct ProductListModel {
    /// product identifier
    pub id: String,
    /// name of the product
    pub name: String,
}

/// ProductDetailsModel defines the layout used for returning a stingle product instance
#[derive(Debug, Serialize)]
pub struct ProductDetailsModel {
    /// product identifier
    pub id: String,
    /// name of the product
    pub name: String,
    /// description of the product
    pub description: String,
}
