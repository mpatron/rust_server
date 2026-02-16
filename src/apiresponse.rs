use axum::{response::{Response, IntoResponse}, Json, http::StatusCode};
use serde::Serialize;

// here we show a type that implements Serialize + Send
#[derive(Serialize)]
struct Message {
    message: String
}

enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}
