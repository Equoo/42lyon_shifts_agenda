build:
	docker build -t foyer-shifts .

run:
	docker run -it --rm --name foyer-shifts-test-run -p 8080:8080 foyer-shifts

preparedb:
	docker run \
		--rm \
		--detach \
		--name test-mariadb \
		-v ./init.sql:/docker-entrypoint-initdb.d/init.sql \
		-e MARIADB_ROOT_PASSWORD=wawa \
		-p 3307:3306 \
		mariadb:latest
	echo "DATABASE_URL=mariadb://foyer:wawa@127.0.0.1:3307/foyer" > .env

stopdb:
	docker stop test-mariadb || true

rmdb: stopdb
	docker rm test-mariadb || true

brun: build run
