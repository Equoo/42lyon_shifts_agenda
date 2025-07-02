build:
	docker build -t foyer-shifts .

run:
	docker run -it --rm --name foyer-shifts-test-run -p 8080:8080 foyer-shifts

brun: build run
