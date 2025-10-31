FROM rust:alpine

WORKDIR /usr/src/foyer-shifts
COPY . .

# Installer tout ce qu’il faut pour un build musl statique
RUN apk add --no-cache \
    musl-dev \
    make \
    nodejs \
    npm \
    pkgconfig \
    libressl-dev  # <- indispensable pour les libs statiques

# (optionnel) build du frontend
# RUN cd frontend && npm install && npm run build && cd ..

ENV SQLX_OFFLINE=true
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run -- --dev"]

