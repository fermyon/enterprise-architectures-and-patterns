#! /bin/bash

set -euo pipefail

pushd src
pushd stats-generator
spin registry push ttl.sh/spin-todo-stats-generator:24h --build
popd
pushd migrations
spin registry push ttl.sh/spin-todo-migrations:24h --build
popd
pushd http-api
spin registry push ttl.sh/spin-todo-api:24h --build
popd 
popd
echo "Done"