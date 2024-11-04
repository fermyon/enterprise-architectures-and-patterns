use std::fmt::Display;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{conversions::IntoBody, IntoResponse, ResponseBuilder},
    pg::{DbValue, Decode},
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateTaskModel {
    pub content: String,
}

impl TryFrom<&[u8]> for CreateTaskModel {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice::<CreateTaskModel>(value)
            .with_context(|| "Could not deserialize value into CreateAndUpdateItemModel")
    }
}
impl Display for CreateTaskModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Updated Item: {}", self.content)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Task {
    pub id: Uuid,
    pub content: String,
    pub done: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task: {}", self.content)
    }
}

impl IntoResponse for Task {
    fn into_response(self) -> spin_sdk::http::Response {
        ResponseBuilder::new(200)
            .header("content-type", "application/json")
            .body(self.into_body())
            .build()
    }
}

impl IntoBody for Task {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec_pretty(&self).unwrap()
    }
}

impl From<&Vec<DbValue>> for Task {
    fn from(value: &Vec<DbValue>) -> Self {
        Self {
            id: Uuid::parse_str(String::decode(&value[0]).unwrap().as_str()).unwrap(),
            content: String::decode(&value[1]).unwrap(),
            done: bool::decode(&value[2]).unwrap_or_default(),
        }
    }
}

impl Task {
    pub(crate) fn new(model: CreateTaskModel) -> Self {
        // roll a new id
        let id = Uuid::new_v4();
        Self {
            id: id,
            content: model.content.clone(),
            done: false,
        }
    }

    pub(crate) fn existing(id: Uuid, model: &Task) -> Self {
        Self {
            id,
            content: model.content.clone(),
            done: model.done,
        }
    }
}

pub(crate) struct Tasks {
    all: Vec<Task>,
}

impl IntoBody for Tasks {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec_pretty(&self.all).unwrap()
    }
}

impl Tasks {
    pub(crate) fn new(items: Vec<Task>) -> Self {
        Self { all: items }
    }
}

impl IntoResponse for Tasks {
    fn into_response(self) -> spin_sdk::http::Response {
        ResponseBuilder::new(200)
            .header("content-type", "application/json")
            .body(self.into_body())
            .build()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Stats {
    pub all: Vec<Stat>,
}

impl Stats {
    pub(crate) fn new(items: Vec<Stat>) -> Self {
        Self { all: items }
    }
}

impl IntoBody for Stats {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec_pretty(&self.all).unwrap()
    }
}

impl IntoResponse for Stats {
    fn into_response(self) -> spin_sdk::http::Response {
        ResponseBuilder::new(200)
            .header("content-type", "application/json")
            .body(self.into_body())
            .build()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Stat {
    pub date: String,
    pub open_tasks: i64,
    pub done_tasks: i64,
}

impl IntoResponse for Stat {
    fn into_response(self) -> spin_sdk::http::Response {
        ResponseBuilder::new(200)
            .header("content-type", "application/json")
            .body(self.into_body())
            .build()
    }
}

impl IntoBody for Stat {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec_pretty(&self).unwrap()
    }
}

impl From<&Vec<DbValue>> for Stat {
    fn from(value: &Vec<DbValue>) -> Self {
        Self {
            date: String::decode(&value[0]).unwrap(),
            open_tasks: i64::decode(&value[1]).unwrap_or_default(),
            done_tasks: i64::decode(&value[2]).unwrap_or_default(),
        }
    }
}
