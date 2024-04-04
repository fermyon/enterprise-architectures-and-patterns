.PHONY: database-up
database-up:
	docker run -d -e MYSQL_USER=spin -e MYSQL_PASSWORD=spin -e MYSQL_ROOT_PASSWORD=secure-pw -e MYSQL_DATABASE=spin -p 3306:3306 --name mysql mysql:latest
	echo "Waiting for DB to become ready..."
	sleep 3
	
.PHONY: database-seed
database-seed:
	docker build . -t mysql-seed
	docker run --rm -e MYSQL_PWD=spin --link mysql mysql-seed

.PHONY: datbase-down
database-down:
	docker rm -f mysql

.PHONY: build
build:
	spin build

.PHONY: run
run:
	spin up

