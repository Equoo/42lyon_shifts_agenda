#! /bin/bash

# docker compose up database -d
#
# if ! cargo sqlx &> /dev/null; then
# 	echo "sqlx-cli could not be found, installing..."
# 	cargo install sqlx-cli --no-default-features --features mysql
# else
# 	echo "sqlx-cli is already installed."
# fi
#
# sleep 3

# DATABASE_URL=mariadb://root:debug@127.0.0.1:3307/foyer cargo sqlx prepare
docker compose build backend
docker compose up -d
