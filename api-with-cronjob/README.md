# HTTP API with CronJob

This sample illustrates how you can use the [Spin Command Trigger](https://github.com/fermyon/spin-trigger-command) to one time commands with Spin.

In the context of Kubernetes we can turn one-time commands into Jobs and CronJobs to perform individual tasks once or on a schedule.

The sample consists of two Spin Apps:

* API: A simple HTTP API that interacts with a key-value store
* CRON: A command app which wipes data from the key-value store

The API exposes the following endpoints

* `GET /value` - Returns the value of a counter from key-value store
* `POST /value` - Increments the counter in key-value store by `1`
* `DELETE /value` - Removes the counter from the key-value store
* `GET /` - Prints available API endpoints

The Command App leverages Spin's key-value store API to load all keys available in the key-value store and deletes them.

## Supported Platforms

- Local (`spin up`)
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

To use this sample you must have

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine
- [Command Trigger Plugin](https://github.com/fermyon/spin-trigger-command) must be installed


## Running the Sample

### Local (`spin up`)

Follow the steps outlined below to run the API and the Command App:

- Start the API using `spin up -f spin.toml --runtime-config-file ./local.rtc.toml`
- Increment the counter by sending `POST` requests to `/value`: `curl -X POST localhost:3000/value`
- Check the value of the counter using `curl localhost:3000/value` (you should see a value other than `0`)
- Run the Command App from a clean terminal instance using `spin up -f spin-cron.toml --runtime-config-file ./local.rtc.toml`
- Check the value of the counter using `curl localhost:3000/value` (it should now return `0`)

### Running on Kubernetes with SpinKube

The [`k8s`](./k8s) folder contains a `deploy.sh` script that you can use to deploy the sample to your Kubernetes cluster.

> You must have [SpinKube](https://spinkube.dev) installed

The script itself deploys the following artifacts to your cluster:

- Redis will be deployed to the `redis` namespace
- The Runtime Configuration File will be stored in the `rtc` secret in the `default` namespace
- The API will be deployed to the `default` namespace
- The Command App will be deployed as a CronJob to the `default` namespace and will be executed every 2nd minute
