#!/bin/bash

docker run \
	--rm \
	--detach \
	--name test-mariadb \
	-v ./init.sql:/docker-entrypoint-initdb.d/init.sql \
	-e MARIADB_ROOT_PASSWORD=debug \
	-p 3307:3306 \
	mariadb:latest
echo "DATABASE_URL=mariadb://foyer:debug@127.0.0.1:3307/foyer" > .env
