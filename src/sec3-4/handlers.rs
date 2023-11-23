use axum::{extract::Extension, http::StatusCode,response::IntoResponse, Json};
use std::sync::Arc;

use crate::repositories::{CreateTodo, TodoRepository};

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repositories): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repositories.create(payload);
    (StatusCode::CREATED, Json(todo))
}

// P134まで対応済(test OK)