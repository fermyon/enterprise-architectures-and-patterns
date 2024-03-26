# Content Negotiation

This folder contains a Content Negotiation implementation written in Rust.

## What is Content Negotiation

Content negotiation is a crucial concept in API development that allows clients and servers to agree on the format and structure of exchanged data. It enables interoperability between different systems by enabling them to communicate using various data formats, such as JSON, XML, or even HTML. Typically, content negotiation occurs during the HTTP request process, where the client expresses its preferences for the data format through request headers like 'Accept'. The server then examines these preferences and selects the most suitable representation of the requested resource, taking into account factors like available formats and the client's stated preferences.

Implementing content negotiation ensures that your APIs can cater to a diverse range of clients with varying capabilities and preferences. By supporting multiple data formats, you can reach a broader audience and accommodate different client needs without requiring separate endpoints for each format. Additionally, content negotiation promotes flexibility and future-proofing, as it allows you to introduce new data formats or modify existing ones without impacting clients that support different formats. Properly implemented content negotiation enhances the usability and accessibility of APIs, fostering seamless communication between clients and servers.

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

## Exposed Endpoints

The API exposes the following endpoints:

- `GET /data`: Returns a list of items
- `GET /data/:id`: Returns a single item (you can either provide any `string` or any int as `:id`)

Supported content types (by specifying the `Content-Type` HTTP-header):

- JSON: (`application/json`)
- YAML: (`application/yaml`)
- XML: (`application/xml`)
- Plain Text: (`text/plain`)

## Running the Sample

## Local (`spin up`)

The following snippet shows how to run the sample on your local machine:

```bash
# Build the sample
spin build

# Run the sample
spin up
```

## Fermyon Cloud

You can deploy this sample to Fermyon Cloud following the steps below:

```bash
# Authenticate
spin cloud login

# Deploy the sample to Fermyon Cloud
spin deploy
```