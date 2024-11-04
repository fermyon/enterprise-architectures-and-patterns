#! /bin/bash

# set flags
set -euo pipefail

pushd src
## Generate stats
pushd stats-generator
spin up --build
popd
