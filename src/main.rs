use std::net::SocketAddr;

use askama::Template;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::{Html, Response},
    routing::get,
    Extension, Router,
};

use sqlx::SqlitePool;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod books;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bookshelf=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = SqlitePool::connect(&database_url)
        .await
        .expect("failed to connect to DATABASE_URL");

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"))
        .merge(books::router())
        .layer(Extension(db))
        .fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Resource not found.")
}
