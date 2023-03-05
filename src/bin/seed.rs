use fake::faker::{job::en::Title, name::en::*};
use fake::Fake;
use sqlx::SqlitePool;

fn fake_book() -> (String, String) {
    let title: String = Title().fake();
    let author: String = Name().fake();
    (title, author)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_conn_str = std::env::var("DATABASE_URL").expect("no DATABASE_URL found in env");
    let pool = SqlitePool::connect(&db_conn_str)
        .await
        .expect("can't connect to database");

    let mut conn = pool.acquire().await.expect("error acquiring pool");

    let n = 50;
    let books: Vec<(String, String)> = (0..n).map(|_| fake_book()).collect();
    for book in books {
        let author_id = sqlx::query!("INSERT INTO authors(name) values ($1)", book.1)
            .execute(&mut conn)
            .await?
            .last_insert_rowid();

        sqlx::query!(
            "INSERT INTO books(title, author_id) values ($1, $2)",
            book.0,
            author_id
        )
        .execute(&mut conn)
        .await?;
    }

    println!("Finshed seeding {} books.", n);

    Ok(())
}
