## Installing k6

```bash
brew install k6
```

## Building and Running the App

In a dedicated terminal session, run the following commands:

```bash
pushd scenario
spin b
spin u
```

## Test Web Dashboard

While tests are running, you can access tests result at [http://127.0.0.1:5665](http://127.0.0.1:5665).

## Running Smoke Test

### Run smoke tests against plain text endpoint use 
```bash
K6_WEB_DASHBOARD=true k6 run smoke-test.js
```

### Run smoke tests against JSON endpoint use 
```bash
K6_WEB_DASHBOARD=true k6 run -e JSON=1 smoke-test.js
```

## Running Stress Test

### Run stress tests against plain text endpoint use 
```bash
K6_WEB_DASHBOARD=true k6 run stress-test.js
```

### Run stress tests against JSON endpoint use 
```bash
K6_WEB_DASHBOARD=true k6 run -e JSON=1 stress-test.js
```

## Running Breakpoint Test

### Run breakpoint tests against plain text endpoint use 
```bash
K6_WEB_DASHBOARD=true k6 run breakpoint-test.js
```

### Run breakpoint tests against JSON endpoint use 
```bash
K6_WEB_DASHBOARD=true k6 run -e JSON=1 breakpoint-test.js
```