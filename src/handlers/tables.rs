use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

use crate::models::database::Table;
use crate::models::response::GetSeatsResponse;
use crate::utils::response_builder::{SuccessResponseBuilder, TableErrorResponseBuilder};
use crate::AppDatabase;

pub async fn get_seats(
    State(app_database): State<Arc<AppDatabase>>,
    Path(table_id): Path<u32>,
) -> Response {
    match sqlx::query_as!(Table, "SELECT id, seats FROM tables WHERE id = ?", table_id)
        .fetch_one(&app_database.connection_pool)
        .await
    {
        Ok(table) => Json(GetSeatsResponse { seats: table.seats }).into_response(),

        Err(err) => {
            let err_resp = err.get_seats_err(table_id);
            eprintln!(
                "=> {} - {}:\n{}",
                "get_table",
                &err_resp.msg,
                err.to_string()
            );
            err_resp.into_response()
        }
    }
}

pub async fn add_table(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<Table>,
) -> Response {
    match sqlx::query("INSERT INTO tables (id, seats) VALUES (?, ?)")
        .bind(body.id)
        .bind(body.seats)
        .execute(&app_database.connection_pool)
        .await
    {
        Ok(result) => result.add_table_response(body.id, body.seats),

        Err(err) => {
            let err_resp = err.add_table_err(body);
            eprintln!(
                "=> {} - {}:\n{}",
                "add_table",
                &err_resp.msg,
                err.to_string()
            );
            err_resp.into_response()
        }
    }
}
