use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    Json,
};
use sqlx::Row;
use std::sync::Arc;

use crate::models::{AppState, CreateUrlRequest, CreateUrlResponse};

pub async fn shorten_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUrlRequest>,
) -> Result<Json<CreateUrlResponse>, StatusCode> {

    let short_id = nanoid::nanoid!(6);

    let result = sqlx::query("INSERT INTO urls (id, original_url) VALUES (?, ?)")
        .bind(&short_id)
        .bind(&payload.original_url)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let response = CreateUrlResponse {
                short_id: short_id.clone(),
                short_url: format!("http://127.0.0.1:3000/{}", short_id),
            };
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn redirect_url(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    
    let result = sqlx::query("SELECT original_url FROM urls WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(row)) => {
            let original_url: String = row.get("original_url");
            Ok(Redirect::to(&original_url))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}