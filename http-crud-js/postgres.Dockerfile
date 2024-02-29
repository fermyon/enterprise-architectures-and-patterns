FROM postgres:alpine as pg
WORKDIR /app
COPY scripts/pg/init.sh /docker-entrypoint-initdb.d
COPY scripts/pg/init.sql ./scripts/db/init.sql

