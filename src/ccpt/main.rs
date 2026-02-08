use colored::Colorize;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@localhost/postgres".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("Failed to create database connection pool !");

    println!(
        "{} database connection pool started started.",
        "✔".green().bold()
    );

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate schema !");

    println!("{} database schema migration done.", "✔".green().bold());

    let infra = infrastructure::create_infra(pool);

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let port: u16 = port.parse().expect("Port should be valid range !");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address !");

    println!("{} Listening on {}.", "✔".green().bold(), addr);

    axum::serve(listener, infra.into_make_service()).await?;

    Ok(())
}
