# Distributed ToDo-app

This is a distributed ToDo-app implementation based on several Spin Apps:

- [HTTP API](./src/http-api/): Exposing API Endpoints for creating, listing and toggling tasks
- [Migrations](./src/migrations/): A Job to initialize database, seed data and perform migrations
- [Stats Generator](./src/stats-generator/): A CronJob for generating simple stats using all tasks in the database

## Exposed HTTP API Endpoints

The HTTP API exposes the following endpoints:

- `GET /tasks`: Retrieve a list of all tasks
- `POST /tasks`: Create a new task
- `GET /tasks/:id`: Retrieve a single task using its identifier
- `POST /tasks/toggle/:id`: Toggle the state of a particular task (open|done) using its identifier
- `GET /stats`: Retrieve all stats

## Running locally

### Prerequisites

 - `docker` CLI must be installed (local DB is hosted in a container)
 - `spin` CLI must be installed
 - The Spin CLI `trigger-command` plugin (canary) must be installed
 - The ports `8080` and `3000` may not be allocated on your system
   - You can use different ports by updating `./run-local.sh` and `./local.toml`

### Running the todo Spin App locally

You can find two scripts in this folder (next to `README.md`) called [`run-local.sh`](./run-local.sh) and [`run-cron-local.sh`](./run-cron-local.sh).

The database is served locally using a docker container. To create the database, seed sample data and start the API execute the following:

```bash
./run-local.sh
```

To run the cron job on your local machine, run the following command:

```bash
./run-cron-local.sh
```

All apps read and write from the same SQLite database (which will be created in this folder upon starting the app on your local machine for the first time) -> `./sqlite_db.db`. All Spin Apps leverage the same *runtime configuration file* [`local.toml`](./local.toml) to do so.

For each execution of the cron job, new stats are written to the `Stats` table and available at `GET localhost:3000/stats`.

```bash
curl -iX GET localhost:3000/stats
HTTP/1.1 200 OK
content-type: application/json
transfer-encoding: chunked
date: Tue, 29 Oct 2024 10:12:36 GMT

[
  {
    "date": "2024-10-29 10:12:33",
    "open_tasks": 11,
    "done_tasks": 14
  }
]
```

## Deploying to Kubernetes

### Prerequisites

- You must have access to a Kubernetes Cluster
- `kubectl` must be installed and must point to your Kubernetes cluster
- [SpinKube](https://spinkube.dev) must be deployed to your Kubernetes cluster

### Creating and Distributing Spin Apps as OCI artifacts

Before deploying the different Spin Apps to your Kubernetes cluster, you must package and distribute the Spin Apps as OCI artifacts. To do so, you can use the [`distribute-apps.sh`](./distribute-apps.sh) script as shown here:

```bash
./distribute.sh
```

> NOTE: The OCI artifacts are stored in `ttl.sh` an anonymous and ephemeral OCI compliant registry. According to the tags specified in `distribute-apps.sh` and in Kubernetes deployment manifests, those artifacts remain available for 24 hours

### Deploying the todo Spin App to Kubernetes

1. Deploy the Database to Kubernetes `kubectl apply -f ./kubernetes/db.yaml`
2. Wait for the DB to be ready `kubectl wait --for=condition=Ready pod -l app=postgres`
3. Deploy the Migrations Job to Kubernetes `kubectl apply -f ./kubernetes/migrations.yaml`
4. Deploy the HTTP API to Kubernetes `kubectl apply -f ./kubernetes/api.yaml`
5. Deploy the CronJob to Kubernetes `kubectl apply -f ./kubernetes/stats-generator.yaml`