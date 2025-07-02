FROM rust:alpine

WORKDIR /usr/src/foyer-shifts
COPY . .

RUN apk add --no-cache musl-dev sqlite make
RUN make preparedb
RUN cargo install --path .

CMD ["foyer-shifts"]
