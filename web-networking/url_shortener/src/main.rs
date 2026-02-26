use axum::{
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

mod handlers;
mod models;

use models::AppState;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let db_url = "sqlite://sqlite.db?mode=rwc";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            original_url TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/shorten", post(handlers::shorten_url))
        .route("/{id}", get(handlers::redirect_url))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}