build_css:
	npx @tailwindcss/cli -i ./tailwind/input.css -o ./resources/css/style.css --minify

build_server:
	docker build -t foyer-shifts .

build: build_css build_server

.PHONY: build_css build_server build
.PHONY: tailwind run preparedb stopdb rmdb brun

tailwind:
	npx @tailwindcss/cli -i ./tailwind/input.css -o ./resources/css/style.css --watch

run:
	docker run -it --rm --name foyer-shifts-test-run -v ./resources/js:/usr/src/foyer-shifts/resources/js -v ./resources/css:/usr/src/foyer-shifts/resources/css -v ./index.html:/usr/src/foyer-shifts/index.html -p 8080:8080 foyer-shifts

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
