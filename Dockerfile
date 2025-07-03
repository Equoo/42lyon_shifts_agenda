FROM rust:alpine

WORKDIR /usr/src/foyer-shifts
COPY . .

RUN apk add --no-cache musl-dev sqlite make
RUN cargo install --path .

CMD ["foyer-shifts"]
