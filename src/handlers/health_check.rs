use axum::response::{IntoResponse, Json, Response};

use crate::models::response::SuccessResponse;

pub async fn health_checker() -> Response {
    Json(SuccessResponse {
        msg: "I'm healthy!".to_string(),
    })
    .into_response()
}
