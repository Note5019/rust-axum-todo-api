use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    time_helper::IntoTimerHelperShared,
    todo::{
        entities::Todos,
        error::{APIError, IntoErrorResponse},
        model::TodoAdding,
        repositories::SharedTodoRepository,
    },
};

pub async fn create_todo(
    Json(body): Json<TodoAdding>,
    todo_repository: SharedTodoRepository,
    time_helper: IntoTimerHelperShared,
) -> impl IntoResponse {
    let temp_todo = Todos::new(body.topic.clone(), Arc::clone(&time_helper));

    let todo = match todo_repository
        .add_todo(temp_todo)
        .await
    {
        Ok(todo) => todo,
        Err(e) => return APIError::AddingTodoError(e).error().into_response(),
    };

    (StatusCode::CREATED, Json(todo.to_model())).into_response()
}
