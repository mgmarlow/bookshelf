DATABASE_URL = "sqlite:bookshelf.db"

install:
	cargo install cargo-watch sqlx-cli

watch:
	cargo watch -x run

db-create:
	sqlx db create --database-url=$(DATABASE_URL)

db-drop:
	sqlx db drop --database-url=$(DATABASE_URL)

db-migrate:
	sqlx migrate run --database-url=$(DATABASE_URL)

db-reset:
	sqlx db reset --database-url=$(DATABASE_URL)

db-seed:
	DATABASE_URL=$(DATABASE_URL) cargo run --bin seed
