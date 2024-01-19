use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use serde::Serialize;

use super::database::Items;

#[derive(Serialize)]
pub struct GenericSuccessResponse {
    //Status code 200 is returned implicitly with into_response()
    pub msg: String,
}
pub struct GenericErrorResponse {
    pub msg: String,
    pub status_code: u16,
}

#[derive(Serialize)]
pub struct GetSeatsResponse {
    pub seats: u32,
}

#[derive(Serialize)]
pub struct ItemsResponse {
    pub items: Vec<Items>,
}

// A bit of a catch all implementation for a struct, used to pipe
// back the server error msg along with a status code to the client
impl IntoResponse for GenericErrorResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(self.msg))
            .unwrap()
    }
}

#[derive(Serialize)]
pub struct SuccessRowsReponse {
    pub msg: String,
    pub rows: u64,
}
