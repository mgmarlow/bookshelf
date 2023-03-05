DATABASE_URL = "sqlite:bookshelf.db"

watch:
	cargo watch -x run

db-create:
	sqlx db create --database-url=$(DATABASE_URL)

db-drop:
	sqlx db drop --database-url=$(DATABASE_URL)

db-migrate:
	sqlx migrate run --database-url=$(DATABASE_URL)

db-reset: db-drop db-create db-migrate
