#![allow(unused)]
use anyhow::Result;
use chrono::expect;
use dotenv::dotenv;
use reqwest::{Client, Response};
use rstest::rstest;
use serde_json::json;

#[path = "../src/models/response.rs"]
mod response;
use response::{GenericErrorResponse, GenericSuccessResponse, GetSeatsResponse};

#[path = "../src/models/database.rs"]
mod database;
use database::Items;

async fn get_test_server() -> (Client, String) {
    // Need env vars for testing as well
    dotenv().ok();
    let app_host = std::env::var("APP_HOST").expect("APP_HOST env var not set!");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT env var not set!");
    let addr = format!("http://{}:{}", app_host, app_port);
    println!("\n=> Host: {}\n", addr,);
    (Client::new(), addr)
}

#[tokio::test]
async fn test_health() {
    let (client, host) = get_test_server().await;
    let route = &"/health".to_string();

    match client.get(host + &route).send().await {
        Ok(response) => {
            assert!(response.status() == 200);
            println!(
                "\n=> Route: {}\n=> Response: {}\n",
                route,
                response.text().await.unwrap()
            );
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            panic!("Failed to get health check response");
        }
    };
}

#[rstest]
#[case(1, 200, 4)]
#[case(99, 500, 0)]
#[tokio::test]
async fn test_get_seats(
    #[case] table_id: u32,
    #[case] expected_status: u16,
    #[case] expected_seats: u32,
) {
    let (client, host) = get_test_server().await;
    let route = "/table/".to_string() + &table_id.to_string();

    match client.get(host + &route).send().await {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            if expected_status == 200 {
                let json_resp = response.json::<GetSeatsResponse>().await.unwrap();
                assert_eq!(json_resp.seats, expected_seats);

                println!(
                    "\n=> Route: {}\n=> Response for table {}: {:?}\n",
                    route, table_id, json_resp
                );
            } else {
                println!(
                    "\n=> Route: {}\n=> Intended error response: {}\n",
                    route,
                    response.text().await.unwrap()
                );
            }
        }
        Err(err) => {
            eprintln!("\n=> Route: {}\n=> Uninteded error: {}\n", route, err);
            panic!("Failed to get number of seats response");
        }
    };
}

#[rstest]
#[case(10000, 10000, 200)]
#[case(10000, 10000, 500)]
#[tokio::test]
async fn test_add_table(#[case] table_id: u32, #[case] seats: u32, #[case] expected_status: u16) {
    let (client, host) = get_test_server().await;
    let route = "/table/add".to_string();

    match client
        .put(host + &route)
        .json(&database::Table {
            id: table_id,
            seats: seats,
        })
        .send()
        .await
    {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            if expected_status == 200 {
                let json_resp = response
                    .json::<response::GenericSuccessResponse>()
                    .await
                    .unwrap();
                assert_eq!(
                    json_resp.msg,
                    format!(
                        "Sucessfully created new table {} with {} seats",
                        table_id, seats
                    )
                );

                println!(
                    "\n=> Route: {}\n=> Response for table {}: {:?}\n",
                    route, table_id, json_resp
                );
            } else {
                println!(
                    "\n=> Route: {}\n=> Intended error response: {}\n",
                    route,
                    response.text().await.unwrap()
                );
                delete_table_by_id(table_id).await; // Cleanup
            }
        }

        Err(err) => {
            eprintln!("\n=> Route: {}\n=> Unintended error: {}\n", route, err);
            panic!("Failed to get add table response");
        }
    };
}

async fn delete_table_by_id(table_id: u32) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/table/delete/".to_string() + &table_id.to_string();

    client.delete(host + &route).send().await
}

#[rstest]
#[case(10000, 200)]
#[tokio::test]
async fn test_delete_table_by_id(#[case] table_id: u32, #[case] expected_status: u16) {
    match delete_table_by_id(table_id).await {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response
                .json::<response::GenericSuccessResponse>()
                .await
                .unwrap();

            assert_eq!(
                json_resp.msg,
                format!("Sucessfully deleted table {}", table_id)
            );

            println!(
                "\n=> Route: /delete/{}\n=> Response for table {}: {:?}\n",
                table_id, table_id, json_resp
            );
        }

        Err(err) => {
            eprintln!(
                "\n=> Route: /delete/{}\n=> Unintended error: {}\n",
                table_id, err
            );
            panic!("Failed to get delete table response");
        }
    }
}
