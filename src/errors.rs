use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AppError {
    ServerSideError,
    ClientSideError(String),
}

pub fn internal_error<E>(_err: E) -> AppError {
    AppError::ServerSideError
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::ServerSideError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            Self::ClientSideError(message) => {
                (StatusCode::BAD_REQUEST, format!("Bad request: {}", message))
            }
        };
        (status, Json(json!({"message": err_msg}))).into_response()
    }
}
