use anyhow::{Context, Result};
use spin_sdk::{
    pg::{Connection, Decode, ParameterValue},
    variables,
};

const SQL_INSERT_STATS: &str = "INSERT INTO Stats (open, done) VALUES ($1, $2)";
const SQL_GET_TASK_STATS: &str = r#"SELECT done FROM Tasks"#;

fn get_connection() -> Result<Connection> {
    let connection_string = variables::get("connection_string")?;
    Connection::open(&connection_string)
        .with_context(|| "Error establishing connection to PostgreSQL database")
}

pub struct Stats {
    pub open: i64,
    pub done: i64,
}

fn main() -> Result<()> {
    println!("Generating stats");
    let connection = get_connection()?;
    let row_set = connection.query(SQL_GET_TASK_STATS, &[])?;
    let stats = row_set
        .rows
        .iter()
        .fold(Stats { open: 0, done: 0 }, |mut acc, stat| {
            if bool::decode(&stat[0]).unwrap() {
                acc.done += 1;
            } else {
                acc.open += 1;
            }
            acc
        });
    let parameters = [
        ParameterValue::Int64(stats.open),
        ParameterValue::Int64(stats.done),
    ];
    connection
        .execute(SQL_INSERT_STATS, &parameters)
        .with_context(|| "Error while writing stats")?;
    println!("Done.");
    Ok(())
}
