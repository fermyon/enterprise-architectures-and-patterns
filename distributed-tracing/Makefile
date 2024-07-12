.PHONY: delete-container

delete-container:
	@if docker ps -q -f name=jaeger | grep -q .; then \
	    echo "Stopping and removing container jaeger..."; \
	    docker stop jaeger; \
	    docker rm jaeger; \
	else \
	    echo "Container jaeger does not exist or is not running."; \
	fi

.PHONY run:

run: delete-container
	docker run -d -p 16686:16686 -p 4317:4317 -p 4318:4318 -e COLLECTOR_OTLP_ENABLED=true --name jaeger jaegertracing/all-in-one:latest;\
	export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318;\
	spin up --build
