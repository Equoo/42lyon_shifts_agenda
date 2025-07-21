FROM rust:alpine

WORKDIR /usr/src/foyer-shifts
COPY . .

RUN apk add --no-cache musl-dev make nodejs npm
RUN cd frontend && npm install && npm run build && cd ..
ENV SQLX_OFFLINE=true
RUN cargo install --path .

CMD ["foyer-shifts"]
