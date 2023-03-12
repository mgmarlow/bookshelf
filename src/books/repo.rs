use serde::Deserialize;
use sqlx::SqlitePool;

use crate::books::Book;

#[derive(Deserialize)]
pub struct CreateBook {
    title: String,
    author: String,
}

pub async fn find_book(db: &SqlitePool, id: i64) -> Result<Book, sqlx::Error> {
    let book = sqlx::query_as!(
        Book,
        "SELECT id, title, author, completed_at FROM books where books.id = $1",
        id
    )
    .fetch_one(db)
    .await?;

    Ok(book)
}

pub async fn get_books(db: &SqlitePool) -> Result<Vec<Book>, sqlx::Error> {
    let books = sqlx::query_as!(Book, "SELECT id, title, author, completed_at FROM books")
        .fetch_all(db)
        .await?;

    Ok(books)
}

pub async fn create_book(db: &SqlitePool, book: &CreateBook) -> Result<i64, sqlx::Error> {
    let mut conn = db.acquire().await?;

    let id = sqlx::query!(
        "INSERT INTO books (title, author) VALUES (?1, ?2)",
        book.title,
        book.author
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn destroy_book(db: &SqlitePool, id: i64) -> Result<i64, sqlx::Error> {
    let mut conn = db.acquire().await?;

    sqlx::query!("DELETE FROM books WHERE id = $1", id)
        .execute(&mut conn)
        .await?;

    Ok(id)
}
