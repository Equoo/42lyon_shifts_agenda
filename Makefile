build:
	docker build -t foyer-shifts .

run:
	docker run -it --rm --name foyer-shifts-test-run -p 8080:8080 foyer-shifts

preparedb:
	sqlite3 database.db < database.sql
	echo "DATABASE_URL=sqlite://database.db" > .env

brun: build run
