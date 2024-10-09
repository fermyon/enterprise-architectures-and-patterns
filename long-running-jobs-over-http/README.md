# Long Running Jobs over HTTP

This folder demonstrates how you can add support for long-running jobs over HTTP using Spin and its `mqtt` capabilities.

## What is Long Running Jobs over HTTP

In some situations you may want to perform data processing which takes longer than usual HTTP requests and could result in users facing timeouts. By using the `mqtt` capabilities provided by Spin, you can move the time-consuming actions (or jobs) to a different (background) process.

For demonstration purposes, this application uses [Eclipse Mosquitto](https://mosquitto.org/) as a message broker to offload time-consuming tasks from the [`API`](./api/) app to either the [`Spin Worker`](./spin-worker/) or the [`Native Worker`](./native-worker/). Upon creating a new job, a unique identifier for the job is created and used to track its status and report back to the callee (using response payload and the response `Location` header).

The API and both workers track the state of the jobs using a SQLite database (located in [`data`](./data/)).

## Exposed Endpoints

The API exposes the following endpoints:

- `GET /jobs/:id` -> To query the status of a particular job using its identifier
- `GET /jobs` -> To retrieve the list of all jobs
- `POST /jobs` -> To start a new job providing the request payload scheme shown below

```json
{
    "input": "lorem ipsum"
}
```
Simulation of job processing is configured to take:

- 1 minute using the [`spin-worker`](./spin-worker/) 
- 2 minutes using the [`native-worker`](./native-worker/)

Both workers will report the job as `Failed` if you provide `FooBar` (will be lower-cased to do comparison) as `input`. Jobs with a different `input` will be processed and reported as `Succeeded`.

## Supported Platforms

- Local (`spin up`) requires a running instance of Mosquitto
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Go](https://go.dev/) installed on your machine
- [Docker](https://docker.com) installed on your machine (for running Mosquitto in a container)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine
  - The [`spin-trigger-mqtt`](https://github.com/spinkube/spin-trigger-mqtt) plugin  

## Running the Sample

### Local (`spin up`)

To run the sample locally, you can use different targets specified in the `Makefile`. As the sample contains two different workers (for demonstration purposes, choose which one you prefer):

```bash
# Start MQTT broker
make start-mosquitto

# run the mass-publisher
make start-api
```

The `spin-worker` leverages the [`spin-trigger-mqtt`](https://github.com/spinkube/spin-trigger-mqtt) trigger for Spin. 

> To run this worker, you must first install the `spin-trigger-mqtt` on your local machine. Consult the [`spin-trigger-mqtt` repository](https://github.com/spinkube/spin-trigger-mqtt) for installation instructions.

Use the following command to start it in a new terminal instance:

```bash
# Start spin-worker
make start-spin-worker
```

Start the `native-worker` is a native worker implementation in Go. Use the following command to start it in a new terminal instance:

```bash
# Start native-worker
make start-native-worker
```
