use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use sqlx::error::Error;
use sqlx::mysql::MySqlQueryResult;

use crate::models::database::Table;
use crate::models::request::{GetItemRequest, TableItem};
use crate::models::response::GenericResponse;

// Success Responses
pub trait TableSuccessResponseBuilder {
    fn add_table_response(self, table_id: u32, seats: u32) -> Response<Body>;
    fn delete_table_by_id_response(self, table_id: u32) -> Response<Body>;
}

impl TableSuccessResponseBuilder for MySqlQueryResult {
    fn add_table_response(self, table_id: u32, seats: u32) -> Response<Body> {
        GenericResponse {
            msg: format!(
                "Sucessfully created new table {} with {} seats",
                table_id, seats
            ),
            status_code: StatusCode::OK.as_u16(),
            rows: Some(self.rows_affected()),
        }
        .into_response()
    }

    fn delete_table_by_id_response(self, table_id: u32) -> Response<Body> {
        // Establishing deleting a row that do not exist is not a 4xx reponse.
        // This assumes the client is not blindly sending delete requests.
        GenericResponse {
            msg: format!("Table {} deleted", table_id),
            status_code: StatusCode::OK.as_u16(),
            rows: Some(self.rows_affected()),
        }
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
        GenericResponse {
            msg: format!("Sucessfully deleted {} item(s)", self.rows_affected()),
            status_code: StatusCode::OK.as_u16(),
            rows: Some(self.rows_affected()),
        }
        .into_response()
    }

    fn add_item_response(self) -> Response<Body> {
        GenericResponse {
            msg: format!("Sucessfully added {} item(s)", self.rows_affected()),
            status_code: StatusCode::OK.as_u16(),
            rows: Some(self.rows_affected()),
        }
        .into_response()
    }
}

// Error responses

// We dont want to expose the sqlx::Error to the client, so we just log it and return a generic error message
pub trait TableErrorResponseBuilder {
    fn get_seats_err(&self, table_id: u32) -> GenericResponse;
    fn add_table_err(&self, body: Table) -> GenericResponse;
    fn delete_table_err(&self, table_id: u32) -> GenericResponse;
}

impl TableErrorResponseBuilder for Error {
    fn get_seats_err(&self, table_id: u32) -> GenericResponse {
        GenericResponse {
            msg: format!(
                "Error attempting to get seating information for table {}",
                table_id
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }

    fn add_table_err(&self, body: Table) -> GenericResponse {
        GenericResponse {
            msg: format!(
                "Error when attempting to insert table {} with {} seats",
                body.id, body.seats
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }

    fn delete_table_err(&self, table_id: u32) -> GenericResponse {
        // This should basically never happen since we allow deleting table ids that dont exist
        GenericResponse {
            msg: format!("Error when attempting to delete table {}", table_id),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }
}

pub trait ItemErrorResponseBuilder {
    fn get_items_err(&self, body: GetItemRequest) -> GenericResponse;
    fn delete_by_id_err(&self, item_id: u32) -> GenericResponse;
    fn delete_item_err(&self, body: TableItem) -> GenericResponse;
    fn add_items_err(&self) -> GenericResponse;
}

impl ItemErrorResponseBuilder for Error {
    fn get_items_err(&self, body: GetItemRequest) -> GenericResponse {
        GenericResponse {
            msg: format!(
                "Error when attempting to get item {} for table {} for customer {}",
                body.item.unwrap_or("None".to_string()),
                body.table_id,
                body.customer_id.unwrap_or("None".to_string())
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }

    fn delete_by_id_err(&self, item_id: u32) -> GenericResponse {
        // This should basically never happen since we allow deleting items that dont exist
        GenericResponse {
            msg: format!("Error when attempting to delete item {}", item_id),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }

    fn delete_item_err(&self, body: TableItem) -> GenericResponse {
        // This should basically never happen since we allow deleting items that dont exist
        GenericResponse {
            msg: format!(
                "Error when attempting to delete item {} from table {}",
                body.item, body.table_id
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }

    fn add_items_err(&self) -> GenericResponse {
        GenericResponse {
            msg: format!("Error when attempting to insert items",),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            rows: None,
        }
    }
}
