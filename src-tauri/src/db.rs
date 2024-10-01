use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

pub async fn establish_connection() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:bookkeeper.db")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}