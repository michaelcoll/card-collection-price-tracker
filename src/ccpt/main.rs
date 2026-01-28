use sqlx::postgres::PgPoolOptions;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("Got: {:?}", row.0);

    Ok(())
}
