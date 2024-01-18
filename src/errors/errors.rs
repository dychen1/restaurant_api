use axum::http::StatusCode;
use sqlx::error::Error;

use crate::models::request::{GetItemRequest, TableItem};
use crate::models::response::GenericErrorResponse;

pub trait ItemErrors {
    fn get_items_err(&self, body: GetItemRequest) -> GenericErrorResponse;
    fn delete_by_id_err(&self, item_id: u32) -> GenericErrorResponse;
    fn delete_item_err(&self, body: TableItem) -> GenericErrorResponse;
    fn add_items_err(&self) -> GenericErrorResponse;
}

impl ItemErrors for Error {
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
            msg: format!("Error encountered when trying to insert items",),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }
}
