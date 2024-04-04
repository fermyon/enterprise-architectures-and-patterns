use anyhow::Context;
use serde::{Deserialize, Serialize};
use spin_sdk::{http::conversions::TryIntoBody, mysql::Decode, pg::DbValue};

pub(crate) trait Validate {
    fn validate(&self) -> bool;
}

#[derive(Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub active: bool,
}

impl Item {
    pub(crate) fn new(id: String, name: String, active: bool) -> Self {
        Self { id, name, active }
    }
}

impl TryIntoBody for Item {
    type Error = anyhow::Error;

    fn try_into_body(self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self).with_context(|| "Error while serializing data into JSON")
    }
}

impl TryFrom<&Vec<DbValue>> for Item {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<DbValue>) -> Result<Self, Self::Error> {
        let id = String::decode(&value[0])?;
        let name = String::decode(&value[1])?;
        let active = bool::decode(&value[2])?;
        Ok(Self { id, name, active })
    }
}

pub struct ListOfItems {
    items: Vec<Item>,
}

impl From<Vec<Item>> for ListOfItems {
    fn from(value: Vec<Item>) -> Self {
        ListOfItems { items: value }
    }
}

impl TryIntoBody for ListOfItems {
    type Error = anyhow::Error;

    fn try_into_body(self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self.items).with_context(|| "Error while serializing data into JSON")
    }
}

#[derive(Deserialize)]
pub struct CreateItemModel {
    pub name: String,
    pub active: bool,
}

impl Validate for CreateItemModel {
    fn validate(&self) -> bool {
        !self.name.is_empty()
    }
}

#[derive(Deserialize)]
pub struct UpdateItemModel {
    pub name: String,
    pub active: bool,
}

impl Validate for UpdateItemModel {
    fn validate(&self) -> bool {
        !self.name.is_empty()
    }
}

#[derive(Deserialize)]
pub struct BatchDeleteModel {
    pub ids: Vec<String>,
}

impl Validate for BatchDeleteModel {
    fn validate(&self) -> bool {
        !self.ids.is_empty() && self.ids.iter().all(|id| uuid::Uuid::parse_str(id).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_item_model_validates_correctly() {
        let sut1 = CreateItemModel {
            name: String::new(),
            active: true,
        };
        let sut2 = CreateItemModel {
            name: String::from("Milk"),
            active: false,
        };

        assert_eq!(sut1.validate(), false);
        assert_eq!(sut2.validate(), true);
    }

    #[test]
    fn update_item_model_validates_correctly() {
        let sut1 = UpdateItemModel {
            name: String::new(),
            active: true,
        };
        let sut2 = UpdateItemModel {
            name: String::from("Milk"),
            active: false,
        };

        assert_eq!(sut1.validate(), false);
        assert_eq!(sut2.validate(), true);
    }

    #[test]
    fn batch_model_validates_correctly() {
        let sut = BatchDeleteModel {
            ids: vec![String::from("3d0ebd7c-8fb8-4f56-8e5b-df5f4a835c2f")],
        };

        assert_eq!(sut.validate(), true)
    }

    #[test]
    fn batch_model_validation_fails_due_to_empty_list_of_ids() {
        let sut = BatchDeleteModel { ids: vec![] };

        assert_eq!(sut.validate(), false)
    }

    #[test]
    fn batch_model_validation_fails_due_to_invalid_id() {
        let sut = BatchDeleteModel {
            ids: vec![
                String::from("foo"),
                String::from("3d0ebd7c-8fb8-4f56-8e5b-df5f4a835c2f"),
            ],
        };

        assert_eq!(sut.validate(), false)
    }
}
