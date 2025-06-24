use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct ErrorResponse {
    pub error: String,
    pub status_code: StatusCode,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!(
                {
                    "error": self.error,
                }
            )),
        )
            .into_response()
    }
}

pub enum APIError {
    AddingTodoError(sqlx::Error),
    UpdatingTodoError(sqlx::Error),
    TodoNotFound(i32),
    FailedToQuery(sqlx::Error),
}

pub trait IntoErrorResponse {
    fn error(&self) -> ErrorResponse;
}

impl IntoErrorResponse for APIError {
    fn error(&self) -> ErrorResponse {
        match self {
            Self::AddingTodoError(err) => ErrorResponse {
                error: format!("Failed to add todo: {:?}", err),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::UpdatingTodoError(err) => ErrorResponse {
                error: format!("Failed to update todo: {:?}", err),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::TodoNotFound(id) => ErrorResponse {
                error: format!("Todo not found: {}", id),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::FailedToQuery(err) => ErrorResponse {
                error: format!("Failed to query: {:?}", err),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}
