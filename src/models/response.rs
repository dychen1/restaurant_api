use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use serde::Serialize;
use sqlx::mysql::MySqlQueryResult;

use super::database::Items;

#[derive(Serialize)]
pub struct SuccessResponse {
    //Status code 200 is returned implicitly with into_response()
    pub msg: String,
}

#[derive(Serialize)]
pub struct GetSeatsResponse {
    pub seats: u32,
}

#[derive(Serialize)]
pub struct ItemsResponse {
    pub items: Vec<Items>,
}

pub struct GenericErrorResponse {
    pub msg: String,
    pub status_code: u16,
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

// Establishing deleting a row that doesnt exist is not an error, ensures idempotency
// This assumes the client is not blindly sending delete requests
// Could also return the number of rows affected in the response
pub trait DeleteResponseMessage {
    fn generate_delete_msg(self, table_id: u32) -> String;
}

impl DeleteResponseMessage for MySqlQueryResult {
    fn generate_delete_msg(self, table_id: u32) -> String {
        let message: String;
        if self.rows_affected() > 0 {
            message = format!(
                "Sucessfully deleted {} item(s) from table {}",
                self.rows_affected(),
                table_id
            );
        } else {
            message = "No rows to delete".to_string()
        };
        return message;
    }
}
