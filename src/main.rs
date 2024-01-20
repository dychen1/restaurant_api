use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use std::sync::Arc;

mod handlers;
mod models;
mod utils;
use handlers::health_check::health_checker;
use handlers::items::{add_items, delete_item, delete_item_by_id, get_items};
use handlers::tables::{add_table, delete_table_by_id, get_seats};
use utils::database_connection::{database_connect, AppDatabase};

#[tokio::main]
async fn main() {
    // Load env vars from .env
    dotenv().ok();
    // Establish a pool of db connections
    let app_database: AppDatabase = match database_connect().await {
        Ok(app_database) => app_database,
        Err(err) => {
            eprintln!("Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Register api routes
    // TODO: implement cascading deletion of items when a table is deleted
    let app = Router::new()
        .route("/health", get(health_checker))
        .route("/table/:id", get(get_seats))
        .route("/table/add", put(add_table))
        .route("/table/delete/:id", delete(delete_table_by_id))
        .route("/items", post(get_items))
        .route("/items/add", put(add_items))
        .route("/items/delete", delete(delete_item))
        .route("/items/delete/:id", delete(delete_item_by_id))
        .with_state(Arc::new(app_database));

    // Build server address
    let app_host = std::env::var("APP_HOST").expect("APP_HOST env var not set!");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT env var not set!");
    let addr = format!("{}:{}", app_host, app_port);
    // Start TCP listener
    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(tcp_listener, app).await.unwrap() // this is our server!
}
