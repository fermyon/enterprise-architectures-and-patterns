#! /bin/bash

mysql -h ${MYSQL_HOST} -u ${MYSQL_USER} -D ${MYSQL_DATABASE} < /app/init.sql