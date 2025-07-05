FROM rust:alpine

WORKDIR /usr/src/foyer-shifts
COPY . .

RUN apk add --no-cache musl-dev sqlite make nodejs npm
ENV DATABASE_URL=mariadb://foyer:wawa@172.17.0.2:3306/foyer
RUN cd frontend && npm run build && cd ..
RUN cargo install --path .

CMD ["foyer-shifts"]
