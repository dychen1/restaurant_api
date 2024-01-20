#![allow(unused)]
use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;

#[path = "../src/models/database.rs"]
mod database;

pub async fn get_test_server() -> (Client, String) {
    // Need env vars for connecting to host
    dotenv().ok();
    let app_host = std::env::var("APP_HOST").expect("APP_HOST env var not set!");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT env var not set!");
    let addr = format!("http://{}:{}", app_host, app_port);
    println!("\n=> Host: {}\n", addr,);
    (Client::new(), addr)
}

pub async fn add_table(table_id: u32, seats: u32) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/table/add".to_string();

    client
        .put(host + &route)
        .json(&database::Table {
            id: table_id,
            seats: seats,
        })
        .send()
        .await
}

pub async fn delete_table_by_id(table_id: u32) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/table/delete/".to_string() + &table_id.to_string();

    client.delete(host + &route).send().await
}
