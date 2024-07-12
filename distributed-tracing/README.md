# Distributed Tracing with Spin and Jaeger

This sample illustrates Spin native integration with OpenTelemetry (OTel). Spin automatically instruments your applications. If the `OTEL_EXPORTER_OTLP_ENDPOINT` is present, the Spin runtime will automatically send distributed traces to the OTLP endpoint.

This sample uses [Jaeger](https://www.jaegertracing.io/) for visualizing distributed traces collected by Spin.

## API Endpoints

The Spin App exposes the following endpoints:

- `GET /` -> Returns an HTTP 200 with a response body
- `GET /slow` -> Sleeps for 5 seconds before returning an HTTP 200
- `GET /kv` -> Interacts with a key-value store before returning an HTTP 200
- `GET /400` -> Returns an HTTP 400
- `GET /404` -> Returns an HTTP 404
- `GET /500` -> Returns an HTTP 500

## Supported Platforms

- Local (`spin up`)
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

To use this sample you must have

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine
- [Docker](https://docker.com) or an alternative container runtime is required to run Jaeger locally

## Running the Sample

### Local (`spin up`)

To run the sample locally, you can use the `run` target defined by the [`Makefile`](./Makefile).

The `run` target does the following:

- If a `jaeger` container is running on your machine, it will be stopped and deleted
- Jaeger All-In-One will be started locally
- Necessary `OTEL_EXPORTER_OTLP_ENDPOINT` environment variable will be set
- The Spin App will be started using `spin up --build`
