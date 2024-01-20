use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::models::response::GenericResponse;

pub async fn health_checker() -> Response {
    GenericResponse {
        msg: "I'm healthy!".to_string(),
        status_code: StatusCode::OK.as_u16(),
        rows: None,
    }
    .into_response()
}
