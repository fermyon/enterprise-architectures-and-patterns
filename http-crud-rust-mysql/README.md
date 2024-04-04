## HTTP CRUD Sample

This is a sample implementation of CRUD (Create, Read, Update, Delete) in Rust.

The sample is using MySQL for persistence and provides the following API endpoints via HTTP:

- `GET /items` - To retrieve a list of all items
- `GET /items/:id` - To retrieve a item using its identifier
- `POST /items` - To create a new item
- `PUT /items/:id` - To update an existing item using its identifier
- `DELETE /items` - To delete multiple items providing an array of identifiers as payload (`{ "ids": []}`)
- `DELETE /items/:id` - To delete an existing item using its identifier

Send data to `POST /items` and `PUT /items/:id` using the following structure:

```jsonc
{
    "name": "item name",
    // boolean (either true or false)
    "active": true
}
```

## Supported Platforms

- Local (`spin up`)
- SpinKube
- Fermyon Platform for Kubernetes
- 
## Prerequisites

To run the sample on your local machine, you must have the following software installed:

 - Latest [Spin](https://developer.fermyon.com/spin) CLI
 - [Docker](https://docker.com)
 - [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)


## Running the Sample

### Local (`spin up`)

To run this sample locally, you can either follow the steps mentioned below or use the corresponding targets specified in the `Makefile`.

```bash
# 1. Start a MySQL container 
## alternatively you can run `make database-up`
docker run -d -e MYSQL_USER=spin \
  -e MYSQL_PASSWORD=spin \
  -e MYSQL_ROOT_PASSWORD=secure-pw \
  -e MYSQL_DATABASE=spin \
  -p 3306:3306 --name mysql \
  mysql:latest

# 2. Build and Run the database seeding container
## alternatively, you can run `make database-seed`
docker build . -t mysql-seed
# snip
docker run --rm -e MYSQL_PWD=spin --link mysql mysql-seed

# 3. Build the Spin App 
## alternatively, you can run `make build`
spin build

# 4. Run the Spin App
## alternatively, you can run `make run`
spin up
```

At this point, you can invoke the API endpoints mentioned above at http://localhost:3000/
