use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

use crate::models::response::GetSeatsResponse;
use crate::models::{database::Table, response::GenericErrorResponse, response::SuccessResponse};
use crate::AppDatabase;

pub async fn get_seats(
    State(app_database): State<Arc<AppDatabase>>,
    Path(table_id): Path<i32>,
) -> Response {
    match sqlx::query_as!(Table, "SELECT id, seats FROM tables WHERE id = ?", table_id)
        .fetch_one(&app_database.connection_pool)
        .await
    {
        Ok(table) => Json(GetSeatsResponse { seats: table.seats }).into_response(),

        Err(err) => {
            let err_resp = format!("Error attempting to get table {}", table_id);
            eprintln!("=> {} - {}:\n{}", "get_table", &err_resp, err.to_string());

            GenericErrorResponse {
                msg: err_resp,
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response()
        }
    }
}

pub async fn add_table(
    State(app_database): State<Arc<AppDatabase>>,
    Json(body): Json<Table>,
) -> Response {
    let table = Table {
        id: body.id,
        seats: body.seats,
    };
    match sqlx::query_as!(
        Table,
        "INSERT INTO tables (id, seats) VALUES (?, ?)",
        table.id,
        table.seats
    )
    .execute(&app_database.connection_pool)
    .await
    {
        Ok(_) => Json(SuccessResponse {
            msg: format!(
                "Sucessfully created new table {} with {} seats",
                table.id, table.seats
            ),
        })
        .into_response(),

        Err(err) => {
            let err_resp = format!(
                "Error when inserting table {} with {} seats",
                table.id, table.seats
            );
            eprintln!("=> {} - {}:\n{}", "add_table", &err_resp, err.to_string());

            GenericErrorResponse {
                msg: err_resp,
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response()
        }
    }
}
