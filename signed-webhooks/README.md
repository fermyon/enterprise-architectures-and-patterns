# Signed Webhooks using the WebAssembly Component Model

This sample demonstrates how to implement signed webhooks by extracting signing and verification logic into a dedicated WebAssembly Component and re-using it from within WebAssembly Components written in different programming languages.

## What are Signed Webhooks?

Webhooks are automated messages sent from one application to another when a specific event occurs. They enable real-time communication and trigger actions in response to events, eliminating the need for continuous polling.

Signing the payload of webhooks is crucial for ensuring the integrity and authenticity of the data being transmitted between applications. By signing the payload, you can verify that the data received from a webhook hasn't been tampered with during transit and originates from a trusted source. This security measure helps prevent various forms of attacks, such as data tampering or injection, which could potentially compromise the integrity and reliability of the system. 

## Components of this sample

This sample consists of three major components:

- [`hmac`](./hmac/): The WebAssembly Component providing an impementation for signing and verifying using HMAC 
- [`webhook-producer`](./webhook-producer/): Spin App written in Rust, using the `hmac` component to sign webhook payloads
- [`webhook-consumer`](./webhook-consumer/): Spin App written in Python, using the `hmac` component to verify integrity of received payloads

## Supported Platforms

- Local (`spin up`)
- Fermyon Cloud
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
  - `cargo component` install it via `cargo install cargo-component`
  - `wasm-tools` install it via `cargo install wasm-tools`
- [Python](https://www.python.org/) installed on your machine
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine

## Running the Sample

### Local (`spin up`)

To run the sample on your local machine, three terminal instances are required. Follow the steps below, to build all components and start the `webhook-producer` and the `webhook-consumer`:

```bash
# Build all components
make build-all
```

```bash
# Start the webhook-producer
make start-producer
pushd webhook-producer;\
        spin up --listen 127.0.0.1:3000 --sqlite @migrations.sql
~/dev/enterprise-architectures-and-patterns/signed-webhooks/webhook-producer ~/dev/enterprise-architectures-and-patterns/signed-webhooks
Logging component stdio to ".spin/logs/"
Storing default SQLite data to ".spin/sqlite_db.db"

Serving http://127.0.0.1:3000
Available Routes:
  webhook-producer: http://127.0.0.1:3000 (wildcard)

# You can terminate the producer at any point using CTRL+C
```

Use the 2nd terminal instance to start the `webhook-consumer`:

```bash
# Start the webhook-consumer
make start-consumer
pushd webhook-consumer;\
        spin up --listen 127.0.0.1:3002
~/dev/enterprise-architectures-and-patterns/signed-webhooks/webhook-consumer ~/dev/enterprise-architectures-and-patterns/signed-webhooks
Logging component stdio to ".spin/logs/"
Storing default key-value data to ".spin/sqlite_key_value.db"

Serving http://127.0.0.1:3002
Available Routes:
  webhook-consumer: http://127.0.0.1:3002 (wildcard)

# You can terminate the consumer at any point using CTRL+C
```

Use the 3rd terminal instance to register the consumer with the producer and to fire the webhook:

```bash
# Register the consumer with the producer
make register-consumer

# Fire the webhook
make fire-webhook
```

At this point, you should check `stdout` of the `webhook-producer` and the `webhook-consumer` to see the payload signature has been verified:

```bash
# producer logs for registration
PRODUCER: Invoking CONSUMER for verfication (handshake) along with random key data: RqJcWxsJDKjgINLWWiBwYEV3ROg7LaXD1Xzz
PRODUCER: Handshake with PRODUCER was successful.

# producer logs for firing the webhook
PRODUCER: Loading all CONSUMERS from database
PRODUCER: Sending signed payload to CONSUMER http://localhost:3002/target
PRODUCER: CONSUMER http://localhost:3002/target responded with status 200

# consumer logs for registration
CONSUMER: Received RqJcWxsJDKjgINLWWiBwYEV3ROg7LaXD1Xzz upon registering for webhooks with PRODUCER.
CONSUMER: Stored key data in key value store

# Consumer logs for webhook invocation
CONSUMER: Received tag  2DFB6D2540B1307E6F67F206FA17600667E1805B99CAE3759B1BEE22B630D0F3
CONSUMER: Loaded key data from key-value store: b'RqJcWxsJDKjgINLWWiBwYEV3ROg7LaXD1Xzz'
CONSUMER: Verifying integrity of payload received from PRODUCER...
-------------------
CONSUMER: Payload verification result: True
-------------------
CONSUMER: Responding with HTTP 200
```

