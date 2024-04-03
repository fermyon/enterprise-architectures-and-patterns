## HTTP CRUD Sample

This is a sample implementation of CRUD (Create, Read, Update, Delete) in Go.

The sample is using SQLite for persistence and provides the following API endpoints via HTTP:

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
 - [Node.js](https://nodejs.org)

## Running this Sample 

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
  http-crud-js-sqlite: http://127.0.0.1:3000 (wildcard)
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
Uploading http-crud-js-sqlite version 0.1.0 to Fermyon Cloud...
Deploying...
App "http-crud-js-sqlite" accesses a database labeled "crud"
    Would you like to link an existing database or create a new database?: Create a new database and link the app to it
What would you like to name your database?
What would you like to name your database?
    Note: This name is used when managing your database at the account level. The app "http-crud-js-sqlite" will refer to this database by the label "crud".
    Other apps can use different labels to refer to the same database.: sincere-mulberry
Creating database named 'sincere-mulberry'
Waiting for application to become ready.......... ready

View application:   https://http-crud-js-sqlite-jcmbpezb.fermyon.app/
Manage application: https://cloud.fermyon.com/app/http-crud-js-sqlite

# Ensure tables are created in the new database (here sincere-mulberry)
spin cloud sqlite execute --database sincere-mulberry @migrations.sql
```