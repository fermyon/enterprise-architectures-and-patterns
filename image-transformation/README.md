# Image Transformation

This folder contains a Spin App illustrating how one could leverage existing libraries from the language ecosystem (here crates) within Spin Apps. The app in this folder consists of two components. 

An API written in Rust and a simple frontend built with HTML5 and JavaScript. The API uses [photon_rs](https://docs.rs/photon-rs/latest/photon_rs/) for applying all sorts of transformations to an image.

## Supported Platforms

- Local (`spin up`)
- Fermyon Cloud
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

To use this sample you must have

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine

## Running the Sample

### Local (`spin up`)

To run the sample locally, follow the steps shown in the snippet below:

```bash
# Build the project
spin build

# Run the sample
spin up
Logging component stdio to ".spin/logs/"

Serving http://127.0.0.1:3000
Available Routes:
  api: http://127.0.0.1:3000/api (wildcard)
  frontend: http://127.0.0.1:3000 (wildcard)
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
Uploading image-transformation version 0.1.0 to Fermyon Cloud...
Deploying...
Waiting for application to become ready......... ready

View application:   https://image-transformation-zbxgelnm.fermyon.app/
  Routes:
  - api: https://image-transformation-zbxgelnm.fermyon.app/api (wildcard)
  - frontend: https://image-transformation-zbxgelnm.fermyon.app (wildcard)
Manage application: https://cloud.fermyon.com/app/image-transformation
```

## Screenshot

![Image Transformation](./screenshot.png)