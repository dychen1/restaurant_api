use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;
use sqlx::QueryBuilder;
use std::sync::Arc;

use crate::models::request::{AddItemsRequest, GetItemsRequest, TableItem};
use crate::models::response::ItemsResponse;
use crate::models::response::{GenericErrorResponse, SuccessResponse};
use crate::AppDatabase;

pub async fn get_items(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<GetItemsRequest>,
) -> Response {
    let mut query = QueryBuilder::new("SELECT * FROM items WHERE table_id = ");
    query.push_bind(body.table_id);

    if body.item.is_some() {
        query.push(" AND item = ");
        query.push_bind(body.item.unwrap());
    } else if body.customer_id.is_some() {
        query.push(" AND customer_id = ");
        query.push_bind(body.customer_id.unwrap());
    }

    match query
        .build_query_as()
        .fetch_all(&app_database.connection_pool)
        .await
    {
        Ok(rows) => Json(ItemsResponse { items: rows }).into_response(),

        Err(err) => GenericErrorResponse {
            msg: format!(
                "Database error when querying for items: \"{}\"",
                err.to_string(),
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
        .into_response(),
    }
}

pub async fn delete_item_by_id(
    State(app_database): State<Arc<AppDatabase>>,
    Path(id): Path<u32>,
) -> Response {
    match sqlx::query!("DELETE FROM items WHERE id = ?", id)
        .execute(&app_database.connection_pool)
        .await
    {
        Ok(_) => Json(SuccessResponse {
            msg: format!("Sucessfully deleted item {}", id),
        })
        .into_response(),

        Err(err) => GenericErrorResponse {
            msg: format!(
                "Database error when querying for items: \"{}\"",
                err.to_string(),
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
        .into_response(),
    }
}

pub async fn delete_item(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<TableItem>,
) -> Response {
    let mut query = QueryBuilder::new("DELETE FROM items WHERE table_id = ");
    query
        .push_bind(&body.table_id)
        .push(" AND item = ")
        .push_bind(&body.item);

    if body.customer_id.is_some() {
        query.push(" AND customer_id = ");
        query.push_bind(body.customer_id.unwrap());
    }

    // Set to only delete first occurance of an item
    match query
        .push(" LIMIT 1")
        .build()
        .execute(&app_database.connection_pool)
        .await
    {
        Ok(_) => Json(SuccessResponse {
            msg: format!(
                "Sucessfully deleted {} from table {}",
                body.item, body.table_id
            ),
        })
        .into_response(),

        Err(err) => GenericErrorResponse {
            msg: format!(
                "Database error when attempting to delete item {} from table {}: \"{}\"",
                body.item,
                body.table_id,
                err.to_string(),
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
        .into_response(),
    }
}

pub async fn add_items(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<AddItemsRequest>,
) -> Response {
    // TODO: Handle bind limit by performing multiple queries
    // Mysql bind limit over number of fields we're binding
    const MYSQL_BIND_LIMIT: usize = 65535 / 4; // .take() expects an usize
    let n = body.to_add.len();

    // Using QueryBuilder because sqlx does not support bulk insert by vector
    match QueryBuilder::new("INSERT INTO items (table_id, item, cook_time, customer_id) ")
        .push_values(
            body.to_add.into_iter().take(MYSQL_BIND_LIMIT),
            |mut builder, item| {
                builder
                    .push_bind(item.table_id)
                    .push_bind(item.item)
                    .push_bind(rand::thread_rng().gen_range(5..=15)) // Static random time for cook time
                    .push_bind(item.customer_id);
            },
        )
        .build()
        .execute(&app_database.connection_pool)
        .await
    {
        Ok(_) => Json(SuccessResponse {
            msg: format!("Sucessfully added {} new items", n),
        })
        .into_response(),

        Err(err) => GenericErrorResponse {
            msg: format!(
                "Database error when inserting items: \"{}\"",
                err.to_string()
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
        .into_response(),
    }
}
