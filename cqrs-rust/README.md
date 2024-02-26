# CQRS Sample written in Rust

This folder contains a fairly simple CQRS implementation written in Rust.

## Prerequisites

To use this sample you must have

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine

## Running the Sample locally

You can run the sample locally using the following commands:

```bash
# Build the project
spin build

# Run the sample
spin up --sqlite @migration.sql
Logging component stdio to ".spin/logs/"
Storing default SQLite data to ".spin/sqlite_db.db"

Serving http://127.0.0.1:3000
Available Routes:
  cqrs-rust: http://127.0.0.1:3000 (wildcard)
```
