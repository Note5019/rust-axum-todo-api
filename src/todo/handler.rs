use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app_state::AppState,
    todo::{
        entities::Todos,
        error::{APIError, IntoErrorResponse},
        model::{TodoAdding, TodoModel},
        repositories::{SharedTodoRepository, TodoRepository},
    },
};

pub async fn create_todo(
    State(app): State<Arc<AppState>>,
    Json(body): Json<TodoAdding>,
) -> impl IntoResponse {
    let todo_repository: SharedTodoRepository = TodoRepository::creation(app.db.clone());
    let temp_todo = Todos::new(body.topic.clone(), Arc::clone(&app.time_helper));

    let todo = match todo_repository.add_todo(temp_todo).await {
        Ok(todo) => todo,
        Err(e) => return APIError::AddingTodoError(e).error().into_response(),
    };

    (StatusCode::CREATED, Json(todo.to_model())).into_response()
}

pub async fn get_todos(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    let todo_repository: SharedTodoRepository = TodoRepository::creation(app.db.clone());
    let todos = match todo_repository.get_todos().await {
        Ok(todos) => todos,
        Err(e) => return APIError::FailedToQuery(e).error().into_response(),
    };

    let todo_models: Vec<TodoModel> = todos.into_iter().map(|t| t.to_model()).collect();

    (StatusCode::OK, Json(todo_models)).into_response()
}
