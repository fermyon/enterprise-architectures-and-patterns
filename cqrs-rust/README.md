# Command and Query Responsibility Segregation (CQRS)

This folder contains a simple Command and Query Responsibility Segregation (CQRS) implementation written in Rust.

## What is Command and Query Responsibility Segregation (CQRS)

CQRS is a software architectural pattern that separates the responsibility of handling commands (requests that change the system's state) from handling queries (requests that fetch data without modifying state). In a CQRS architecture, commands are handled by a separate component or layer known as the Command side, while queries are handled by another component or layer called the Query side. This segregation allows each side to be optimized independently, as they often have different scalability, performance, and optimization requirements.

On the Command side, operations are focused on enforcing business rules, validation, and updating the state of the system. This side typically utilizes a domain-driven design approach to model the business logic effectively. On the Query side, the emphasis is on efficiently retrieving data to fulfill read requests from clients. This side often employs de-normalized data models and specialized data storage mechanisms optimized for fast read access. By separating the concerns of commands and queries, CQRS promotes a clearer separation of concerns and can lead to improved scalability, performance, and maintainability in complex software systems.

## Supported Platforms

- Local (`spin up`)
- Fermyon Cloud
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

To use this sample you must have

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine

## Running the Sample

### Local (`spin up`)

To run the sample locally, you must provide the `local.toml` as runtime config file as shown in the snippet below:

```bash
# Build the project
spin build

# Run the sample
spin up --sqlite @migrations.sql --runtime-config-file ./local.toml
Logging component stdio to ".spin/logs/"
Storing default SQLite data to ".spin/sqlite_db.db"

Serving http://127.0.0.1:3000
Available Routes:
  cqrs-rust: http://127.0.0.1:3000 (wildcard)
```

### Fermyon Cloud

You can deploy this sample to Fermyon Cloud following the steps below:

```bash
# Authenticate
spin cloud login

# Deploy the sample to Fermyon Cloud
# This will ask if a new database should be created or an existing one should be used
# Answer the question with "create a new database"
spin deploy
Uploading cqrs-rust version 0.1.0 to Fermyon Cloud...
Deploying...
App "cqrs-rust" accesses a database labeled "cqrs"
    Would you like to link an existing database or create a new database?: Create a new database and link the app to it
What would you like to name your database?
    Note: This name is used when managing your database at the account level. The app "cqrs-rust" will refer to this database by the label "cqrs".
    Other apps can use different labels to refer to the same database.: little-fish
Creating database named 'little-fish'
Waiting for application to become ready........ ready

View application:   https://cqrs-rust-3avdbcka.fermyon.app/
Manage application: https://cloud.fermyon.com/app/cqrs-rust


# Ensure tables are created in the new database (here little-fish)
spin cloud sqlite execute --database little-fish @migrations.sql
```