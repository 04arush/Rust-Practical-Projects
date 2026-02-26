use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::{CreateTodo, Todo, UpdateTodo};

// CREATE
pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title) VALUES (?) RETURNING *",
        payload.title
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

// READ ALL
pub async fn list_todos(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

// READ ONE
pub async fn get_todo(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todo {
        Some(t) => Ok(Json(t)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// UPDATE
pub async fn update_todo(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let current_todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let new_title = payload.title.unwrap_or(current_todo.title);
    let new_completed = payload.completed.unwrap_or(current_todo.completed);

    let updated_todo = sqlx::query_as!(
        Todo,
        "UPDATE todos SET title = ?, completed = ? WHERE id = ? RETURNING *",
        new_title,
        new_completed,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_todo))
}

// DELETE
pub async fn delete_todo(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}