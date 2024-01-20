use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use sqlx::error::Error;
use sqlx::mysql::MySqlQueryResult;

use crate::models::database::Table;
use crate::models::request::{GetItemRequest, TableItem};
use crate::models::response::{GenericErrorResponse, GenericSuccessResponse, SuccessRowsReponse};

// Success Responses
pub trait TableSuccessResponseBuilder {
    fn add_table_response(self, table_id: u32, seats: u32) -> Response<Body>;
    fn delete_table_by_id_response(self, table_id: u32) -> Response<Body>;
}

impl TableSuccessResponseBuilder for MySqlQueryResult {
    fn add_table_response(self, table_id: u32, seats: u32) -> Response<Body> {
        Json(GenericSuccessResponse {
            msg: format!(
                "Sucessfully created new table {} with {} seats",
                table_id, seats
            ),
        })
        .into_response()
    }

    fn delete_table_by_id_response(self, table_id: u32) -> Response<Body> {
        // Establishing deleting a row that do not exist is not a 4xx reponse.
        // This assumes the client is not blindly sending delete requests.
        Json(GenericSuccessResponse {
            msg: format!("Sucessfully deleted table {}", table_id),
        })
        .into_response()
    }
}

pub trait ItemSuccessResponseBuilder {
    fn delete_item_response(self) -> Response<Body>;
    fn add_item_response(self) -> Response<Body>;
}

impl ItemSuccessResponseBuilder for MySqlQueryResult {
    fn delete_item_response(self) -> Response<Body> {
        // Same assumptions as above for deleting rows
        Json(SuccessRowsReponse {
            msg: format!("Sucessfully deleted {} item(s)", self.rows_affected()),
            rows: self.rows_affected(),
        })
        .into_response()
    }

    fn add_item_response(self) -> Response<Body> {
        Json(SuccessRowsReponse {
            msg: format!("Sucessfully added {} item(s)", self.rows_affected()),
            rows: self.rows_affected(),
        })
        .into_response()
    }
}

// Error responses
pub trait TableErrorResponseBuilder {
    fn get_seats_err(&self, table_id: u32) -> GenericErrorResponse;
    fn add_table_err(&self, body: Table) -> GenericErrorResponse;
    fn delete_table_err(&self, table_id: u32) -> GenericErrorResponse;
}

impl TableErrorResponseBuilder for Error {
    fn get_seats_err(&self, table_id: u32) -> GenericErrorResponse {
        GenericErrorResponse {
            msg: format!(
                "Error attempting to get seating information for table {}",
                table_id
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    fn add_table_err(&self, body: Table) -> GenericErrorResponse {
        GenericErrorResponse {
            msg: format!(
                "Error when attempting to insert table {} with {} seats",
                body.id, body.seats
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    fn delete_table_err(&self, table_id: u32) -> GenericErrorResponse {
        // This should basically never happen since we allow deleting table ids that dont exist
        GenericErrorResponse {
            msg: format!("Error when attempting to delete table {}", table_id),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }
}

pub trait ItemErrorResponseBuilder {
    fn get_items_err(&self, body: GetItemRequest) -> GenericErrorResponse;
    fn delete_by_id_err(&self, item_id: u32) -> GenericErrorResponse;
    fn delete_item_err(&self, body: TableItem) -> GenericErrorResponse;
    fn add_items_err(&self) -> GenericErrorResponse;
}

impl ItemErrorResponseBuilder for Error {
    fn get_items_err(&self, body: GetItemRequest) -> GenericErrorResponse {
        GenericErrorResponse {
            msg: format!(
                "Error when attempting to get item {} for table {} for customer {}",
                body.item.unwrap_or("None".to_string()),
                body.table_id,
                body.customer_id.unwrap_or("None".to_string())
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    fn delete_by_id_err(&self, item_id: u32) -> GenericErrorResponse {
        // This should basically never happen since we allow deleting items that dont exist
        GenericErrorResponse {
            msg: format!("Error when attempting to delete item {}", item_id),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    fn delete_item_err(&self, body: TableItem) -> GenericErrorResponse {
        GenericErrorResponse {
            msg: format!(
                "Error when attempting to delete item {} from table {}",
                body.item, body.table_id
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    fn add_items_err(&self) -> GenericErrorResponse {
        GenericErrorResponse {
            msg: format!("Error when attempting to insert items",),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }
}
