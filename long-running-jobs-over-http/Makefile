.PHONY: start-mosquitto
start-mosquitto:
	docker run -d -p 1883:1883 -p 9001:9001 -v ./mosquitto:/mosquitto/config:rw  --name mosquitto eclipse-mosquitto

.PHONY: stop-mosquitto
stop-mosquitto:
	docker rm -f mosquitto

.PHONY: start-api
start-api:
	pushd api;\
	spin up --sqlite @migrations.sql --runtime-config-file ./local.toml --build

.PHONY: start-spin-worker
start-spin-worker:
	pushd spin-worker;\
	spin up --build
	

.PHONY: start-native-worker
start-native-worker:
	pushd native-worker;\
	go run main.go --release
