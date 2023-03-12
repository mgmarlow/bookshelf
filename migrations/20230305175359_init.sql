CREATE TABLE IF NOT EXISTS books (
       id INTEGER PRIMARY KEY,
       title TEXT NOT NULL UNIQUE,
       author TEXT NOT NULL,
       completed_at TEXT
);

CREATE TABLE IF NOT EXISTS collections (
       id INTEGER PRIMARY KEY,
       name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS collections_books (
       collection_id INTEGER,
       book_id INTEGER,
       FOREIGN KEY(collection_id) REFERENCES collections(id),
       FOREIGN KEY(book_id) REFERENCES books(id)
);
