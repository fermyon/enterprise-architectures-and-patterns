# Enterprise Architectures & Patterns

This repository contains a collection of enterprise architectures and patterns, to illustrate the potential of [Spin](https://github.com/fermyon/spin) and WebAssembly (Wasm) for building real-world applications.

Each folder contains a self-contained example, including a high-level explanation of the pattern/architecture and instructions on how to build and run the particular Spin App(s).

## Getting Started with Spin

If you haven't used Spin before, we highly recommend exploring the [Fermyon Developer Home](https://developer.fermyon.com) to get started. There you'll find everything from installing Spin on your local machine, over language-specific guides, to advanced techniques when building serverless Wasm applications.

## Architectures & Patterns in this Repository

### CRUD APIs
- [Go CRUD API with persistence in SQLite](./http-crud-go-sqlite/)
- [JavaScript CRUD API with persistence in PostgreSQL](./http-crud-js-pg/)
- [JavaScript CRUD API with persistence in SQLite](./http-crud-js-sqlite/)
- [Rust CRUD API with persistence in MySQL](./http-crud-rust-mysql/)

### Command and Query Responsibility Segregation (CQRS)
- [Command and Query Responsibility Segregation (CQRS) in Go](./cqrs-go/)
- [Command and Query Responsibility Segregation (CQRS) in Rust](./cqrs-rust/)
- [Command and Query Responsibility Segregation (CQRS) using Spin Service Chaining in Rust and Go](./cqrs-servicechaining/)

### Patterns for building HTTP APIs
- [Content Negotiation](./content-negotiation-rust/)
- [Cross-Origin Resource Sharing (CORS)](./cors-rust/)
- [Long Running Jobs over HTTP](./long-running-jobs-over-http/)
- [Transparent Caching](./caching-rust/)

### Application Variables (aka Configuration Data)
- [Using the Azure Key Vault Application Variable Provider](./application-variable-providers/azure-key-vault-provider/)
- [Using the Vault Application Variable Provider](./application-variable-providers/vault-provider/)

### Webhooks
- [Signed Webhooks using WebAssembly Component Model](./signed-webhooks/)

### Distributed Application Patterns
- [Polyglot Aggregate Pattern implementation](./aggregate-pattern/)
- [Polyglot Publish-Subscribe](./pub-sub-polyglot/)

### Testing
- [How to load test Spin Apps with Grafana k6](./load-testing-spin-with-k6/)


## Are you looking for a particular pattern or architecture

We do our best to keep the list of enterprise architectures and patterns continuously growing. If you're missing a particular architecture or pattern, [create an issue and let us know](https://github.com/fermyon/enterprise-architectures-and-patterns/issues).
