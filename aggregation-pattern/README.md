# Aggregate Pattern

This folder contains an implementation of the Aggregate Pattern. The sample consists of two backend services (written in Go and TypeScript) and an aggregate (written in Rust).

## What is the Aggregate Pattern

In the context of a microservices architecture, the Aggregate Design Pattern refers to a strategy for composing multiple microservices to fulfill a single business operation or query. Instead of relying on a single microservice to handle complex operations, aggregates distribute the workload across multiple services, each responsible for managing a specific aspect of the operation. This pattern helps in achieving scalability, fault isolation, and autonomy among microservices. By decomposing large operations into smaller, manageable tasks distributed across microservices, the Aggregate Design Pattern enables more efficient and resilient systems in microservices architecture.

## Sample Scenario

For demonstration purposes, take the two backend services [`customers-service`](./customers-service/) and [`incidents-service`](./incidents-service/) as given. Although they expose necessary data directly, developers must issue many requests to create a dashboard displaying information provided by those backend services. 

Instead, an **aggregate** is implemented ([`aggregates-service](./aggregates-service/)) which is responsible for loading data from backend services (via service chaining) and composing a uniform representation for data that should be visualized on the dashboard. 

In addition to calling into backend services using service chaining, the *Aggregates Service* is responsible to transform data according to the use-case.


### Exposed Endpoints

The *Aggregates Service* exposes the following endpoints:

 - `GET /aggregates/dashboard` -> To retrieve data for a dashboard view (this data is constructed by calling into the *Customers Service* and the *Incidents Service* using service chaining)

The *Customers Service* exposes the following endpoints:

 - `GET /customers/top/:limit` -> To retrieve a list of N (specified via `:limit`) customers (sorted by scoring in descending order)
 - `GET /customers/count` -> To retrieve the number of all customers
 - `GET /customers/items` -> To retrieve the list of all customers (sorted by name)
 - `GET /customers/items/:id` -> To retrieve a single customer using its identifier (`:id`)

The *Incidents Service* exposes the following endpoints:

- `GET /incidents/grouped-by-customer` -> to retrieve a list of incidents grouped by customer
- `GET /incidents/items` -> To retrieve the list of all incidents
- `GET /incidents/items/:id` -> To retrieve a single incident using its identifier (`:id`)

## Supported Platforms

- Local (`spin up`) requires a running redis cache
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [TinyGo](https://tinygo.org/) installed on your machine
- [Node.js](https://nodejs.org/) installed on your machine
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine

## Running the Sample

### Local (`spin up`)

To run the sample locally, you must provide the migrations.sql file using the --sqlite flag to provision the database on the first run:

```bash
# Build the project
spin build

# Run the sample
spin up --sqlite @migrations.sql
Logging component stdio to ".spin/logs/"
Storing default SQLite data to ".spin/sqlite_db.db"

Serving http://127.0.0.1:3000
Available Routes:
  aggregates-service: http://127.0.0.1:3000/aggregates (wildcard)
  customers-service: http://127.0.0.1:3000/customers (wildcard)
  incidents-service: http://127.0.0.1:3000/incidents (wildcard)
```