use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use serde::Serialize;

pub struct GenericErrorResponse {
    pub msg: String,
    pub status_code: u16,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    //Status code 200 is returned implicitly with into_response()
    pub msg: String,
}

#[derive(Serialize)]
pub struct GetSeatsResponse {
    pub seats: u32,
}

impl IntoResponse for GenericErrorResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(self.msg))
            .unwrap()
    }
}
