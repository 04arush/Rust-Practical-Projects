use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub struct AppState {
    pub db: SqlitePool,
}

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    pub original_url: String,
}

#[derive(Serialize)]
pub struct CreateUrlResponse {
    pub short_id: String,
    pub short_url: String,
}