.PHONY: start-redis
start-redis:
	docker run --name redis -d -p 6379:6379 redis

.PHONY: stop-redis
stop-redis:
	docker rm -f redis

.PHONY: start-go-subscriber
start-go-subscriber:
	pushd subscriber-go;\
	SPIN_VARIABLE_REDIS_CONNECTION_STRING=redis://localhost:6379 SPIN_VARIABLE_REDIS_CHANNEL=demochannel spin up --build

.PHONY: start-rust-subscriber
start-rust-subscriber:
	pushd subscriber-rust;\
	SPIN_VARIABLE_REDIS_CONNECTION_STRING=redis://localhost:6379 SPIN_VARIABLE_REDIS_CHANNEL=demochannel spin up --build

.PHONY: start-mass-publisher
start-mass-publisher:
	pushd mass-publisher;\
	REDIS_CONNECTION_STRING=redis://localhost:6379 REDIS_CHANNEL=demochannel cargo run --release

.PHONY: start-http-publisher
start-http-publisher:
	pushd http-publisher;\
	SPIN_VARIABLE_REDIS_CONNECTION_STRING=redis://localhost:6379 SPIN_VARIABLE_REDIS_CHANNEL=demochannel spin up --build