# Publish-Subscribe

This folder contains a Publish-Subscribe pattern implemented using different programming languages.

## What is Publish-Subscribe

The Publish-Subscribe pattern is a messaging pattern widely used in distributed systems to facilitate communication between multiple components or modules in a decoupled manner. In this pattern, publishers are responsible for producing messages containing data or events of interest, while subscribers express interest in specific types of messages by subscribing to relevant topics or channels. When a publisher generates a message, it is broadcasted to all subscribed subscribers without the publisher needing to have any knowledge of the subscribers' identities or how they process the messages. This decoupling enables loose coupling between components, making systems more flexible, scalable, and easier to maintain.

Subscribers can react to messages they are interested in by executing predefined actions or processing the data contained within the messages. This pattern is commonly implemented using message brokers or event buses, where publishers send messages to a centralized location and subscribers receive messages from this central hub. By leveraging Publish-Subscribe, you can design systems where components are highly modular and can be easily extended or modified without affecting other parts of the system. Additionally, this pattern supports asynchronous communication, enabling efficient handling of large volumes of messages and improving system responsiveness.

## Supported Platforms

- Local (`spin up`) requires a running redis cache
- SpinKube
- Fermyon Platform for Kubernetes

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine
  - The `wasm32-wasi` target for Rust installed (`rustup target add wasm32-wasi`)
- [TinyGo](https://tinygo.org/) installed on your machine
- [Node.js](https://nodejs.org/) installed on your machine
- [Docker](https://docker.com) installed on your machine (for running Redis in a container)
- [Spin](https://developer.fermyon.com/spin/v2/index) CLI installed on your machine

## Running the Sample

### Local (`spin up`)

To run the sample locally, you can use different targets specified in the `Makefile`. Follow the steps below to continuously publish messages into a Redis channel using the [`mass-publisher`](./mass-publisher-rust/) and have two subscribers [`subscriber-go`](./subscriber-go) and [`subscriber-rust`](./subscriber-rust):

```bash
# Start Redis
make start-redis

# run the mass-publisher
make start-mass-publisher
```

Start the `subscriber-go` in a new terminal instance:

```bash
# Start subscriber-go
make start-subscriber-go
```

Start the `subscriber-rust` in a new terminal instance:

```bash
# Start subscriber-rust
make start-subscriber-rust
```