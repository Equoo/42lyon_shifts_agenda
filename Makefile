build:
	docker build -t foyer-shifts .

run:
	docker rm -f foyer-shifts-test-run || true
	docker run -it --rm --detach --name foyer-shifts-test-run -v ./frontend/build:/usr/src/foyer-shifts/frontend/build -p 8080:8080 foyer-shifts
	cd frontend && npm run start
	docker rm -f foyer-shifts-test-run || true

brun: build run

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

.PHONY: run preparedb stopdb rmdb build