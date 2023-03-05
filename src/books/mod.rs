use askama::Template;
use axum::{response::IntoResponse, routing::get, Extension, Router};
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
    Router::new().route("/books", get(books_index))
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
            vec![]
        }
    };

    let template = IndexTemplate { books };
    HtmlTemplate(template)
}

pub async fn get_books(db: &SqlitePool) -> Result<Vec<Book>, sqlx::Error> {
    let books = sqlx::query_as!(Book, "SELECT id, title, author, completed_at FROM books")
        .fetch_all(db)
        .await?;

    Ok(books)
}
