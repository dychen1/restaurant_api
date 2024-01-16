use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck {
    msg: &'static str,
}

pub async fn health_checker() -> Json<HealthCheck> {
    Json(HealthCheck {
        msg: "I'm healthy!",
    })
}
