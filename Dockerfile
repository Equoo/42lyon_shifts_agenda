FROM rust:alpine

WORKDIR /usr/src/foyer-shifts
COPY . .

RUN cargo install --path .

CMD ["foyer-shifts"]
