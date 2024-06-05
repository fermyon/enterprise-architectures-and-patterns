.PHONY register-consumer:
register-consumer:
	curl -X DELETE http://localhost:3000/registrations
	curl -X POST -H 'Content-Type: application/json' -d '{"url": "http://localhost:3002/target", "event": "*"}' http://localhost:3000/registrations

.PHONY fire-webhook:
	curl -iX POST http://localhost:3000/fire

.PHONY build-all: build-hmac build-producer build-consumer

.PHONY build-hmac:
build-hmac:
	pushd hmac;\
	make build

.PHONY build-producer:
build-producer:
	pushd webhook-producer;\
	make build

.PHONY build-consumer:
build-consumer:
	pushd webhook-consumer;\
	make build

.PHONY start-producer:
start-producer:
	pushd webhook-producer;\
	spin up --listen 127.0.0.1:3000 --sqlite @migrations.sql

.PHONY start-consumer:
start-consumer:
	pushd webhook-consumer;\
	spin up --listen 127.0.0.1:3002

.PHONY install-deps:
install-deps:
	cargo install cargo-binstall
	echo "yes" | cargo binstall wasm-tools --force
	echo "yes" | cargo binstall cargo-component --force

