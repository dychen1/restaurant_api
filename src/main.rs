use axum::{routing::get, Router};
use dotenv::dotenv;
use std::sync::Arc;

mod api;
use api::health_check::health_checker;

mod database_utils;
use database_utils::database_connection::{database_connect, AppDatabase};

mod models;

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

    // Establish api routes
    let app = Router::new()
        .route("/health", get(health_checker))
        .with_state(Arc::new(app_database));

    // Build server address
    let app_host = std::env::var("APP_HOST").expect("APP_HOST env var not set!");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT env var not set!");
    let addr = format!("{}:{}", app_host, app_port);
    // Start TCP listener
    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server hosted on {}", addr);
    axum::serve(tcp_listener, app).await.unwrap() // this is our server!
}
