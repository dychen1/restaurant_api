use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;
use sqlx::QueryBuilder;
use std::sync::Arc;

use crate::models::{
    request::{AddItemsRequest, GetItemRequest, TableItem},
    response::ItemsResponse,
};
use crate::utils::response_builder::{ItemErrorResponseBuilder, ItemSuccessResponseBuilder};
use crate::AppDatabase;

pub async fn get_items(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<GetItemRequest>,
) -> Response {
    let mut query = QueryBuilder::new("SELECT * FROM items WHERE table_id = ");
    query.push_bind(body.table_id);

    let item = body.item.clone();
    if item.is_some() {
        query.push(" AND item = ");
        query.push_bind(item.unwrap());
    }
    let customer_id = body.customer_id.clone();
    if customer_id.is_some() {
        query.push(" AND customer_id = ");
        query.push_bind(customer_id.unwrap());
    };

    match query
        .push(" ORDER BY created_at DESC")
        .build_query_as()
        .fetch_all(&app_database.connection_pool)
        .await
    {
        Ok(rows) => Json(ItemsResponse { items: rows }).into_response(),

        Err(err) => {
            let err_resp = err.get_items_err(body);
            eprintln!("=> {} - {}:\n{}", "get_item", err_resp.msg, err.to_string());
            err_resp.into_response()
        }
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
        Ok(results) => results.delete_item_response(),

        Err(err) => {
            let err_resp = err.delete_by_id_err(id);
            eprintln!(
                "=> {} - {}:\n{}",
                "delete_item_by_id",
                err_resp.msg,
                err.to_string()
            );
            err_resp.into_response()
        }
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

    let body_clone = body.clone();
    if body_clone.customer_id.is_some() {
        query.push(" AND customer_id = ");
        query.push_bind(body_clone.customer_id.unwrap());
    }

    match query
        .push(" ORDER BY created_at DESC")
        .push(" LIMIT 1") // Only delete latest item
        .build()
        .execute(&app_database.connection_pool)
        .await
    {
        Ok(results) => results.delete_item_response(),

        Err(err) => {
            let err_resp = err.delete_item_err(body);
            eprintln!(
                "=> {} - {}:\n{}",
                "delete_item",
                err_resp.msg,
                err.to_string()
            );
            err_resp.into_response()
        }
    }
}

pub async fn add_items(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<AddItemsRequest>,
) -> Response {
    // TODO: Handle bind limit by performing multiple queries
    // Mysql bind limit for number of fields that we can bind
    const MYSQL_BIND_LIMIT: usize = 65535 / 4;

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
        Ok(result) => result.add_item_response(),

        Err(err) => {
            let err_resp = err.add_items_err();
            eprintln!(
                "=> {} - {}:\n{}",
                "add_items",
                err_resp.msg,
                err.to_string()
            );
            err_resp.into_response()
        }
    }
}
