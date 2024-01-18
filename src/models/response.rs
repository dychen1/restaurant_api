use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::mysql::MySqlQueryResult;

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

pub trait ResponseBuilder {
    fn new_add_table_response(self, table_id: u32, seats: u32) -> Response<Body>;
    fn new_delete_item_response(self, table_id: u32) -> Response<Body>;
    fn new_add_item_reseponse(self) -> Response<Body>;
}

impl ResponseBuilder for MySqlQueryResult {
    fn new_delete_item_response(self, table_id: u32) -> Response<Body> {
        // Establishing deleting a row that do not exist is not a 4xx reponse.
        // This assumes the client is not blindly sending delete requests.
        Json(SuccessRowsReponse {
            msg: format!(
                "Sucessfully deleted {} item(s) from table {}",
                self.rows_affected(),
                table_id
            ),
            rows: self.rows_affected(),
        })
        .into_response()
    }

    fn new_add_item_reseponse(self) -> Response<Body> {
        Json(SuccessRowsReponse {
            msg: format!("Sucessfully deleted {} item(s)", self.rows_affected()),
            rows: self.rows_affected(),
        })
        .into_response()
    }

    fn new_add_table_response(self, table_id: u32, seats: u32) -> Response<Body> {
        Json(GenericSuccessResponse {
            msg: format!(
                "Sucessfully created new table {} with {} seats",
                table_id, seats
            ),
        })
        .into_response()
    }
}
