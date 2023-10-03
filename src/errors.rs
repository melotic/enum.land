use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    MissingDomainParam,
    BadResponse,
    InternalServerError(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::InternalServerError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(error) => {
                error!("unhandled internal server error: {:?}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
            AppError::MissingDomainParam => (StatusCode::BAD_REQUEST, "missing q parameter"),
            AppError::BadResponse => (StatusCode::BAD_REQUEST, "bad response"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
