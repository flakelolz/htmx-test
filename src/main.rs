use anyhow::Result;
use axum::{Extension, Router};
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

mod askama;
mod database;
mod handler;

// Test
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let connection_pool = init_db().await?;

    let app = Router::new()
        .nest_service("/", handler::router())
        .nest_service("/css", ServeFile::new("css/styles.css"))
        .layer(Extension(connection_pool));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

// Create a database connection pool. Run any migrations.
pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&connection_pool).await?;
    Ok(connection_pool)
}
