use askama::Template;
use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::{AppendHeaders, Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use sqlx::SqlitePool;

use crate::HtmlTemplate;

use self::repo::*;

mod repo;

pub struct Book {
    id: i64,
    title: String,
    author: String,
    completed_at: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route("/books", get(books_index).post(books_create))
        .route("/books/:id", get(books_show).delete(books_destroy))
        .route("/books/new", get(books_new))
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
    match find_book(&*db, id).await {
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

#[derive(Template)]
#[template(path = "books/new.html")]
struct NewTemplate {}

async fn books_new() -> impl IntoResponse {
    let template = NewTemplate {};
    HtmlTemplate(template).into_response()
}

async fn books_create(
    db: Extension<SqlitePool>,
    Form(form_data): Form<CreateBook>,
) -> impl IntoResponse {
    match create_book(&*db, &form_data).await {
        Ok(id) => (
            StatusCode::NO_CONTENT,
            AppendHeaders([("HX-Redirect", format!("/books/{}", id))]),
        )
            .into_response(),
        Err(_err) => {
            tracing::error!("Error creating book");
            Html("<p>Error creating book</p>").into_response()
        }
    }
}

async fn books_destroy(db: Extension<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    match destroy_book(&*db, id).await {
        Ok(_id) => (
            StatusCode::NO_CONTENT,
            AppendHeaders([("HX-Redirect", "/books")]),
        )
            .into_response(),
        Err(_err) => {
            tracing::error!("Error deleting book");
            (StatusCode::NOT_FOUND, "Book not deleted").into_response()
        }
    }
}
