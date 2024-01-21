use super::database::Items;
use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericResponse {
    pub msg: String,
    pub status_code: u16,
    pub rows: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSeatsResponse {
    pub seats: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemsResponse {
    pub items: Vec<Items>,
}

// Used for everything that is not get_sets or get_items
// In hinde sight, not super necessary since axum::Json has .into_response() implemented
// Still useful for more descriptive error responses
impl IntoResponse for GenericResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(
                serde_json::to_string(&self).unwrap_or_else(|_| "".to_string()),
            ))
            .unwrap()
    }
}
