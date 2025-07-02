use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app_state::AppState,
    todo::{
        entities::Todos,
        error::{APIError, IntoErrorResponse},
        model::{CreateTodo, TodoModel, UpdateTodo},
        repositories::{SharedTodoRepository, TodoRepository},
    },
};

pub async fn create_todo(
    State(app): State<Arc<AppState>>,
    Json(body): Json<CreateTodo>,
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

pub async fn get_todo(State(app): State<Arc<AppState>>, Path(id): Path<i32>) -> impl IntoResponse {
    let todo_repository: SharedTodoRepository = TodoRepository::creation(app.db.clone());
       let todo = match todo_repository.get_todo(id).await {
        Ok(todo) => todo,
        Err(e) => return APIError::FailedToQuery(e).error().into_response(),
    };

    (StatusCode::OK, Json(todo.to_model())).into_response()
}

pub async fn update_todo(
    State(app): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateTodo>,
) -> impl IntoResponse {
    let todo_repository: SharedTodoRepository = TodoRepository::creation(app.db.clone());

    let mut todo = match todo_repository.get_todo(id).await {
        Ok(todo) => todo,
        Err(_) => return APIError::TodoNotFound(id).error().into_response(),
    };

    if let Some(topic) = data.topic {
        todo.topic = topic;
    }

    if let Some(completed) = data.completed {
        todo.completed = completed;
        todo.completed_at = match completed {
            true => Some(app.time_helper.now()),
            false => None,
        }
    }

    todo.updated_at = app.time_helper.now();

    match todo_repository.update_todo(todo).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => APIError::AddingTodoError(e).error().into_response(),
    }
}
