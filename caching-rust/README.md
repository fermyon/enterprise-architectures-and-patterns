# Transparent Cache Sample

This example illustrates how to implement caching when building HTTP APIs.

## What is a Transparent Cache

A transparent cache is a caching mechanism that operates without the explicit involvement or awareness of the end user or client application. In other words, users interacting with a system are unaware that caching is taking place behind the scenes. From the perspective of the user or client application, the caching process is invisible and does not impact the functionality or behavior of the system. Transparent caching is commonly employed in systems where performance optimization is crucial, such as web applications or content delivery networks (CDNs), to reduce latency and server load without affecting user experience.

## Supported Platforms

- Local (`spin up`)
- Fermyon Cloud
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine
  
## Sample Description

The API uses:
 - a key value store as transient cache
 - a sqlite database for persisting data

It exposes the following endpoints

 - `GET /items`                 -> Retrieve a list of items
 - `GET /items/:id`             -> Retrieve an item using its identifier
 - `PUT /items/:id`             -> Modify an existing item using its identifier
 - `DELETE /invalidate-all`     -> Removes all data from the cache

Both `GET` endpoints return data as `application/json`. If data is served from cache, a custom HTTP header (`X-Served-From-Cache`) is sent as part of the HTTP response. 

When updating an item via `PUT`, the corresponding item and the list of all items will be invalidated in cache. Before sending the response of the `PUT` request to the callee, the updated item is stored in cache.

## Running the Sample

### Local (`spin up`)

Follow these steps, to run this sample on your local machine:

```bash
# build the Spin App
spin build

# run the Spin App
spin up --runtime-config-file ./local.toml --sqlite @migrations.sql
```

### Fermyon Cloud

You can deploy this sample to Fermyon Cloud following the steps below:

```bash
# Authenticate
spin cloud login

# Deploy the sample to Fermyon Cloud
# 1. This will ask if a new database should be created or an existing one should be used
# Answer the question with "create a new database"

# 2. This will as if a new key-value store should be created or if an existing one should be used
# Answer the question with "create a new key-value store"
spin deploy
Uploading caching-rust version 0.1.0 to Fermyon Cloud...
Deploying...
App "caching-rust" accesses a database labeled "default"
    Would you like to link an existing database or create a new database?: Create a new database and link the app to it
What would you like to name your database?
    Note: This name is used when managing your database at the account level. The app "caching-rust" will refer to this database by the label "default".
    Other apps can use different labels to refer to the same database.: enthusiastic-plum
Creating database named 'enthusiastic-plum'
App "caching-rust" accesses a key value store labeled "cache"
    Would you like to link an existing key value store or create a new key value store?: Create a new key value store and link the app to it
What would you like to name your database?
    Note: This name is used when managing your database at the account level. The app "caching-rust" will refer to this database by the label "cache".
    Other apps can use different labels to refer to the same database.: mykv
Creating key value store named 'mykv'
Waiting for application to become ready........... ready

View application:   https://caching-rust-8lvuf8gn.fermyon.app/
Manage application: https://cloud.fermyon.com/app/caching-rust

# Ensure tables are created in the new database (here enthusiastic-plum)
spin cloud sqlite execute --database enthusiastic-plum @migrations.sql
```