.PHONY: database-up
database-up:
	docker run --name spin-crud-js-db -d -e POSTGRES_DB=sample -e POSTGRES_USER=timmy -e POSTGRES_PASSWORD=secret -p 5432:5432 spin-crud-js-db:local

.PHONY: datbase-down
database-down:
	docker rm -f spin-crud-js-db

.PHONY: database-build
database-build:
	docker build -f postgres.Dockerfile -t spin-crud-js-db:local .

.PHONY: build
build:
	spin build

.PHONY: run
run:
	SPIN_VARIABLE_DB_CONNECTION_STRING=postgres://timmy:secret@localhost/sample spin up

