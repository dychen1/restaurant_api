use axum::response::{IntoResponse, Json, Response};

use crate::models::response::GenericSuccessResponse;

pub async fn health_checker() -> Response {
    Json(GenericSuccessResponse {
        msg: "I'm healthy!".to_string(),
    })
    .into_response()
}
