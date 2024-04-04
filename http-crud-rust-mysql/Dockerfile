FROM ubuntu:latest
ENV MYSQL_HOST=mysql
ENV MYSQL_USER=spin
ENV MYSQL_PWD=
ENV MYSQL_DATABASE=spin

RUN apt-get update && apt-get install -y \
    mysql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY scripts/mysql/init.sql .
COPY scripts/mysql/entrypoint.sh .
ENTRYPOINT [ "/app/entrypoint.sh" ] 