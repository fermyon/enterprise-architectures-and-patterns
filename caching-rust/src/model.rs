use serde::{Deserialize, Serialize};
pub(crate) struct CacheKey {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateItemModel {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Item {
    pub id: String,
    pub name: String,
}

impl CacheKey {
    pub(crate) fn new(value: String) -> Self {
        CacheKey { value }
    }
}

impl From<Item> for CacheKey {
    fn from(value: Item) -> Self {
        CacheKey::new(format!("type-{}-id-{}", Item::get_cache_type(), value.id).to_lowercase())
    }
}

impl From<(uuid::Uuid, &'static str)> for CacheKey {
    fn from(value: (uuid::Uuid, &'static str)) -> Self {
        CacheKey::new(format!("type-{}-id-{}", value.1, value.0).to_lowercase())
    }
}

impl From<&Vec<Item>> for CacheKey {
    fn from(value: &Vec<Item>) -> Self {
        CacheKey::new(
            value
                .iter()
                .map(|d| d.id.clone())
                .collect::<Vec<String>>()
                .join("_")
                .to_lowercase(),
        )
    }
}

pub trait CacheKeyProvider {
    fn get_cache_type() -> &'static str;
    fn get_cache_key_for_all() -> CacheKey;
}

impl CacheKeyProvider for Item {
    fn get_cache_type() -> &'static str {
        "Item"
    }

    fn get_cache_key_for_all() -> CacheKey {
        CacheKey::new(format!("all-of-type-{}", Item::get_cache_type()))
    }
}
