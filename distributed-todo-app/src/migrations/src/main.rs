use core::str;

use anyhow::{Context, Result};
use rust_embed::Embed;
use spin_sdk::{pg::Connection, variables};

fn get_connection() -> Result<Connection> {
    let connection_string = variables::get("connection_string")?;
    Connection::open(&connection_string)
        .with_context(|| "Error establishing connection to PostgreSQL database")
}

fn main() -> Result<()> {
    println!("Migrating database");
    let connection = get_connection()?;
    for file_path in Asset::iter() {
        let file = Asset::get(&file_path).unwrap();
        let file_contents = str::from_utf8(&file.data)?;
        let statements = file_contents
            .split("\n\n")
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<&str>>();
        println!("Found {} statements in file", statements.len());
        for statement in statements {
            println!("Executing: {}", statement);
            connection.execute(statement, &[])?;
        }
    }
    println!("Done.");
    Ok(())
}

#[derive(Embed)]
#[folder = "scripts"]
#[include("*.sql")]
struct Asset;
