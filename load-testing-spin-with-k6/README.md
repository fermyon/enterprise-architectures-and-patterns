# Load-Testing Spin Apps with k6

> Grafana [k6](https://k6.io/) is an open-source load testing tool that makes performance testing easy and productive for engineering teams. k6 is free, developer-centric, and extensible.

Samples in this folder illustrate how to load test Spin applications.

## Install k6

You can find detailed installation instruction for k6 at https://grafana.com/docs/k6/latest/get-started/installation/. For example, you can install k6 on macOS using the [Homebrew package manager](https://brew.sh/):

```bash
# Install k6
brew install k6
```

## Building and Running the Spin App

To build and run the sample Spin App, run the following commands:

```bash
# Move into the scenario folder
pushd scenario

# Build the Spin App
spin build

# Run the Spin App
spin up
```

## k6 - Test Web Dashboard

While tests are running, you can access tests result at [http://127.0.0.1:5665](http://127.0.0.1:5665).

## Running Smoke Test

### Run smoke tests against plain text endpoint use 

```bash
# Enable dashboard and run smoke tests
K6_WEB_DASHBOARD=true k6 run smoke-test.js
```

### Run smoke tests against JSON endpoint use 

```bash
# Enable dashboard and run smoke tests
K6_WEB_DASHBOARD=true k6 run -e JSON=1 smoke-test.js
```

## Running Stress Test

### Run stress tests against plain text endpoint use 

```bash
# Enable dashboard and run stress tests
K6_WEB_DASHBOARD=true k6 run stress-test.js
```

### Run stress tests against JSON endpoint use 

```bash
# Enable dashboard and run stress tests
K6_WEB_DASHBOARD=true k6 run -e JSON=1 stress-test.js
```

## Running Breakpoint Test

### Run breakpoint tests against plain text endpoint use 

```bash
# Enable dashboard and run breakpoint tests
K6_WEB_DASHBOARD=true k6 run breakpoint-test.js
```

### Run breakpoint tests against JSON endpoint use 

```bash
# Enable dashboard and run breakpoint tests
K6_WEB_DASHBOARD=true k6 run -e JSON=1 breakpoint-test.js
```