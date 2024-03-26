## HTTP CRUD Sample

This is a sample implementation of CRUD (Create, Read, Update, Delete) in JavaScript.

The sample is using PostgreSQL for persistence and provides the following API endpoints via HTTP:

- `GET /items` - To retrieve a list of all items
- `GET /items/:id` - To retrieve a item using its identifier
- `POST /items` - To create a new item
- `PUT /items/:id` - To update an existing item using its identifier
- `DELETE /items/batch` - To delete multiple items providing an array of identifiers as payload
- `DELETE /items/:id` - To delete an existing item using its identifier

Send data to `POST /items` and `PUT /items/:id` using the following structure:

```jsonc
{
    "name": "item name",
    // boolean (either true or false)
    "active": true
}
```

## Prerequisites

To run the sample on your local machine, you must have the following software installed:

 - Latest [Spin](https://developer.fermyon.com/spin) CLI
 - [Docker](https://docker.com)
 - [Node.js](https://nodejs.org)

## Running this sample locally

To run this sample locally, you can either follow the steps mentioned below or use the corresponding targets specified in the `Makefile`.

1. Build the container image for the database using `docker build -f postgres.Dockerfile -t spin-crud-js-db:local .`
2. Run the database container using `docker run --name spin-crud-js-db -d -e POSTGRES_DB=sample -e POSTGRES_USER=timmy -e POSTGRES_PASSWORD=secret -p 5432:5432 spin-crud-js-db:local`
3. Build the Spin App using `spin build`
4. Run the Spin App using `SPIN_VARIABLE_DB_CONNECTION_STRING=postgres://timmy:secret@localhost/sample spin up`

At this point, you can invoke the API endpoints mentioned above at http://localhost:3000/
