export DATABASE_URL=sqlite:bookshelf.db

install:
	cargo install cargo-watch sqlx-cli

watch:
	cargo watch -x run

db-create:
	sqlx db create

db-drop:
	sqlx db drop

db-migrate:
	sqlx migrate run

db-reset:
	sqlx db reset

db-seed:
	cargo run --bin seed
