use std::vec;

use anyhow::{Context, Result};
use spin_sdk::{
    http::{conversions::IntoBody, IntoResponse, Params, Request, Response, ResponseBuilder},
    pg::{Connection, ParameterValue},
    variables,
};
use uuid::Uuid;

use crate::models::{CreateTaskModel, Stat, Stats, Task, Tasks};

const SQL_READ_ALL: &str = "SELECT id, content, done FROM Tasks ORDER BY content ASC";
const SQL_READ_ALL_STATS: &str = "SELECT timestamp, open, done FROM Stats ORDER BY timestamp DESC";
const SQL_READ_BY_ID: &str = "SELECT id, content, done FROM Tasks WHERE id = $1";
const SQL_CREATE: &str = "INSERT INTO Tasks (id, content, done) VALUES ($1, $2, $3)";
const SQL_TOGGLE_TASK_BY_ID: &str = "UPDATE Tasks SET done = $1 WHERE id = $2";

fn get_connection() -> Result<Connection> {
    let connection_string = variables::get("connection_string")?;
    Connection::open(&connection_string)
        .with_context(|| "Error establishing connection to PostgreSQL database")
}
pub(crate) fn get_all_stats(_: Request, _: Params) -> Result<impl IntoResponse> {
    let connection = get_connection()?;
    let row_set = connection.query(SQL_READ_ALL_STATS, &[])?;
    let stats = Stats::new(
        row_set
            .rows
            .iter()
            .map(|item| item.into())
            .collect::<Vec<Stat>>(),
    );
    Ok(stats.into_response())
}
pub(crate) fn get_all_tasks(_: Request, _: Params) -> Result<impl IntoResponse> {
    let connection = get_connection()?;
    let row_set = connection.query(SQL_READ_ALL, &[])?;
    let sessions = Tasks::new(
        row_set
            .rows
            .iter()
            .map(|item| item.into())
            .collect::<Vec<Task>>(),
    );
    Ok(sessions.into_response())
}

pub(crate) fn get_task_by_id(_: Request, params: Params) -> Result<impl IntoResponse> {
    let id = params.get("id").unwrap();
    let id = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(400, "Bad Request")),
    };
    let connection = get_connection()?;
    let parameters = vec![ParameterValue::Str(id.to_string())];
    let row_set = connection.query(SQL_READ_BY_ID, &parameters)?;
    let found = row_set.rows.first();
    Ok(match found {
        Some(row) => Task::from(row).into_response(),
        None => Response::new(404, "Not Found"),
    })
}

pub(crate) fn create_task(req: Request, _: Params) -> Result<impl IntoResponse> {
    let Ok(model) = CreateTaskModel::try_from(req.body()) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    let task = Task::new(model);
    let connection = get_connection()?;

    let parameters = vec![
        ParameterValue::Str(task.id.to_string()),
        ParameterValue::Str(task.content.clone()),
        ParameterValue::Boolean(false),
    ];
    let _ = connection.execute(SQL_CREATE, &parameters)?;
    Ok(ResponseBuilder::new(201)
        .header("content-type", "application/json")
        .body(task.into_body())
        .build())
}

pub(crate) fn toggle_task_by_id(_: Request, params: Params) -> Result<impl IntoResponse> {
    let id = params.get("id").unwrap();
    let id = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(400, "Bad Request")),
    };
    let connection = get_connection()?;
    let parameters = vec![ParameterValue::Str(id.to_string())];
    let row_set = connection.query(SQL_READ_BY_ID, &parameters)?;
    let found = row_set.rows.first();
    let mut found = match found {
        Some(row) => Task::from(row),
        None => return Ok(Response::new(404, "Not Found")),
    };

    found.done = !found.done;
    let parameters = vec![
        ParameterValue::Boolean(found.done),
        ParameterValue::Str(id.to_string()),
    ];
    let _ = connection.execute(SQL_TOGGLE_TASK_BY_ID, &parameters)?;
    let item = Task::existing(id, &found);
    Ok(item.into_response())
}
