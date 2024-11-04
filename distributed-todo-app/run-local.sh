#! /bin/bash

docker stop todo-db 2>/dev/null || true
docker rm todo-db 2>/dev/null || true
docker run --name todo-db -e POSTGRES_DB=todo -e POSTGRES_USER=timmy -e POSTGRES_PASSWORD=secret -p 5432:5432 -d postgres
sleep 2
# set flags
set -euo pipefail
pushd src
## Initialize the database
pushd migrations
spin up --build
popd

## Start the API
pushd http-api
spin up --build
popd