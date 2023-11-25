use anyhow::Ok;
use axum::{
    extract::{Extension,  Path},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use std::{sync::Arc, process::id};

use crate::repositories::{CreateTodo, TodoRepository, self};

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repositories): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repositories.create(payload);
    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse{
    todo!()
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Json(payload): Json<UpadateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> { 
    todo!();
    Ok(StatusCode::OK)
}
pub async fn delete_todo<T: TodoRepository> (
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    todo!()
}
// P137 途中まで