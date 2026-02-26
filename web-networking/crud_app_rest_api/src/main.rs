mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create SQLite connection pool");

    let app = Router::new()
        .route("/todos", post(handlers::create_todo).get(handlers::list_todos))
        .route(
            "/todos/{id}",
            get(handlers::get_todo)
                .patch(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}