# Caching Sample

This example illustrates how to implement caching when building HTTP APIs.

The API uses:
 - a key value store as transient cache
 - a sqlite database for persisting data

## Exposed Endpoints

The API exposes the following endpoints:

 - `GET /items`                 -> Retrieve a list of items
 - `GET /items/:id`             -> Retrieve an item using its identifier
 - `PUT /items/:id`             -> Modify an existing item using its identifier
 - `DELETE /invalidate-all`     -> Removes all data from the cache

Both `GET` endpoints return data as `application/json`. If data is served from cache, a custom HTTP header (`X-Served-From-Cache`) is sent as part of the HTTP response. 

When updating an item via `PUT`, the corresponding item and the list of all items will be invalidated in cache. Before sending the response of the `PUT` request to the callee, the updated item is stored in cache.


## Running the sample locally

Follow these steps, to run this sample on your local machine:

```bash
# build the Spin App
spin build

# run the Spin App
spin up --sqlite @migration.sql
```