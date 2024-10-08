use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;

pub async fn connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");
    pool
}
