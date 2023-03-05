use askama::Template;
use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Router};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::HtmlTemplate;

#[derive(Serialize)]
pub struct Book {
    id: i64,
    title: String,
    author: String,
    completed_at: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route("/books", get(books_index))
        .route("/books/:id", get(books_show))
}

#[derive(Template)]
#[template(path = "books/index.html")]
struct IndexTemplate {
    books: Vec<Book>,
}

async fn books_index(db: Extension<SqlitePool>) -> impl IntoResponse {
    let books = match get_books(&*db).await {
        Ok(books) => books,
        Err(_err) => {
            tracing::error!("Error showing books");
            panic!("error handling todo")
        }
    };

    let template = IndexTemplate { books };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "books/show.html")]
struct ShowTemplate {
    book: Book,
}

async fn books_show(db: Extension<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    let book = match get_book(&*db, id).await {
        Ok(book) => book,
        Err(_err) => {
            tracing::error!("Error fetching book");
            panic!("error handling todo")
        }
    };

    let template = ShowTemplate { book };
    HtmlTemplate(template)
}

pub async fn get_book(db: &SqlitePool, id: i64) -> Result<Book, sqlx::Error> {
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
