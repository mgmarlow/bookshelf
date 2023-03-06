use askama::Template;
use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Router,
};
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
    match get_books(&*db).await {
        Ok(books) => {
            let template = IndexTemplate { books };
            HtmlTemplate(template).into_response()
        }
        Err(_err) => {
            tracing::error!("Error showing books");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Oops! Something went wrong.",
            )
                .into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "books/show.html")]
struct ShowTemplate {
    book: Book,
}

async fn books_show(db: Extension<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    match get_book(&*db, id).await {
        Ok(book) => {
            let template = ShowTemplate { book };
            HtmlTemplate(template).into_response()
        }
        Err(_err) => {
            tracing::error!("Error fetching book");
            (StatusCode::NOT_FOUND, "Book not found").into_response()
        }
    }
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
