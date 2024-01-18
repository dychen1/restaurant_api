use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;
use sqlx::QueryBuilder;
use std::sync::Arc;

use crate::models::database::Items;
use crate::models::request::{AddItemsRequest, ItemsRequest};
use crate::models::response::ItemsResponse;
use crate::models::response::{GenericErrorResponse, SuccessResponse};
use crate::AppDatabase;

pub async fn get_items(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<ItemsRequest>,
) -> Response {
    // let and_condition = match body.items {
    //     Some(items) => (" AND items IN (?)".to_string(), items.join(", ")),
    //     _ => ("".to_string(), "".to_string()),
    // };

    // match QueryBuilder::new("SELECT * FROM items WHERE table_id = ")
    //     .push_bind(body.table_id)
    //     // .push(and_condition.0)
    //     // .push_bind(and_condition.1)
    //     .build_query_as()
    //     .fetch_all(&app_database.connection_pool)
    //     .await
    // {
    //     Ok(rows) => Json(ItemsResponse { items: rows }).into_response(),

    //     Err(err) => GenericErrorResponse {
    //         msg: format!(
    //             "Database error when querying for items: \"{}\"",
    //             err.to_string()
    //         ),
    //         status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    //     }
    //     .into_response(),
    // }

    match sqlx::query_as!(
        Items,
        "SELECT * FROM items WHERE table_id = ? AND item IN (?)",
        body.table_id,
        body.items.join(", "),
    )
    .fetch_all(&app_database.connection_pool)
    .await
    {
        Ok(rows) => Json(ItemsResponse { items: rows }).into_response(),
        Err(err) => GenericErrorResponse {
            msg: format!(
                "Database error when querying for items: \"{}\"",
                err.to_string()
            ),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
        .into_response(),
    }
}

pub async fn get_all_table_items(
    State(app_database): State<Arc<AppDatabase>>,
    Path(table_id): Path<i32>,
) -> Response {
    match sqlx::query_as!(Items, "SELECT * FROM items WHERE table_id = ?", table_id,)
        .fetch_all(&app_database.connection_pool)
        .await
    {
        Ok(rows) => Json(ItemsResponse { items: rows }).into_response(),
        Err(err) => GenericErrorResponse {
            msg: format!(
                "Database error when querying for items: \"{}\"",
                err.to_string()
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
