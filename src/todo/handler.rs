use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    time_helper::IntoTimerHelperShared,
    todo::{
        entities::Todos,
        error::{APIError, IntoErrorResponse},
        model::{TodoAdding, TodoModel},
        repositories::SharedTodoRepository,
    },
};

pub async fn create_todo(
    Json(body): Json<TodoAdding>,
    todo_repository: SharedTodoRepository,
    time_helper: IntoTimerHelperShared,
) -> impl IntoResponse {
    let temp_todo = Todos::new(body.topic.clone(), Arc::clone(&time_helper));

    let todo = match todo_repository.add_todo(temp_todo).await {
        Ok(todo) => todo,
        Err(e) => return APIError::AddingTodoError(e).error().into_response(),
    };

    (StatusCode::CREATED, Json(todo.to_model())).into_response()
}

pub async fn get_todos(todo_repository: SharedTodoRepository) -> impl IntoResponse {
    let todos = match todo_repository.get_todos().await {
        Ok(todos) => todos,
        Err(e) => return APIError::FailedToQuery(e).error().into_response(),
    };

    let todo_models: Vec<TodoModel> = todos.into_iter().map(|t| t.to_model()).collect();

    (StatusCode::OK, Json(todo_models)).into_response()
}
