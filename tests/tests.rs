use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use rstest::rstest;

#[path = "../src/models/response.rs"]
mod response;
use response::{GenericResponse, GetSeatsResponse, ItemsResponse};

#[path = "../src/models/request.rs"]
mod request;
use request::{AddItemsRequest, GetItemRequest, TableItem};

#[path = "../src/models/database.rs"]
mod database;

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
#[case(999, 500, 0)]
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
#[case(999, 10000, 200)]
#[case(999, 10000, 500)]
#[tokio::test]
async fn test_add_table(#[case] table_id: u32, #[case] seats: u32, #[case] expected_status: u16) {
    match add_table(table_id, seats).await {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            if expected_status == 200 {
                let json_resp = response.json::<response::GenericResponse>().await.unwrap();
                assert_eq!(
                    json_resp.msg,
                    format!(
                        "Sucessfully created new table {} with {} seats",
                        table_id, seats
                    )
                );

                println!(
                    "\n=> Route: /table/add\n=> Response for table {}: {:?}\n",
                    table_id, json_resp
                );
            } else {
                println!(
                    "\n=> Route: /table/add\n=> Intended error response: {}\n",
                    response.text().await.unwrap()
                );
                let _ = delete_table_by_id(table_id).await; // Cleanup
            }
        }

        Err(err) => {
            eprintln!("\n=> Route: /table/add\n=> Unintended error: {}\n", err);
            panic!("Failed to get add table response");
        }
    };
}

#[rstest]
#[case(999, 1, 200)]
#[case(999, 0, 200)]
#[tokio::test]
async fn test_delete_table_by_id(
    #[case] table_id: u32,
    #[case] rows_affected: u64,
    #[case] expected_status: u16,
) {
    if rows_affected == 1 {
        let _ = add_table(table_id, 1).await; // Add table for deletion
    }
    match delete_table_by_id(table_id).await {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<GenericResponse>().await.unwrap();

            assert_eq!(json_resp.rows, Some(rows_affected));

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

#[rstest]
#[case(GetItemRequest{table_id: 1, item: None, customer_id: None}, 4, 200)]
#[case(GetItemRequest{table_id: 1, item: Some("Bun Cha".to_string()), customer_id: None}, 2, 200)]
#[case(GetItemRequest{table_id: 1, item: Some("Bun Cha".to_string()), customer_id: Some("Anthony Bourdain".to_string())}, 1, 200)]
#[case(GetItemRequest{table_id: 999, item: None, customer_id: None}, 0, 200)]
#[tokio::test]
async fn test_get_items(
    #[case] request: GetItemRequest,
    #[case] expected_rows: usize,
    #[case] expected_status: u16,
) {
    match get_items(request).await {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<ItemsResponse>().await.unwrap();
            assert_eq!(json_resp.items.len(), expected_rows);

            println!(
                "\n=> Route: /items\n=> Response for get items: {:?}\n",
                json_resp
            );
        }
        Err(err) => {
            eprintln!("\n=> Route: /items\n=> Unintended error: {}\n", err);
            panic!("Failed to get add items response");
        }
    }
}

// #[rstest]
// #[case(AddItemsRequest{to_add: vec![ TableItem{table_id: 999, item: "Burger".to_string(), customer_id: Some("Bob".to_string())}] } , 1, 200)]
// #[tokio::test]
// async fn test_add_item(
//     #[case] request: AddItemsRequest,
//     #[case] expected_rows: u64,
//     #[case] expected_status: u16,
// ) {
//     let _ = add_table(999, 1).await; // Add table for item

//     match add_item(request).await {
//         Ok(response) => {
//             assert!(response.status().as_u16() == expected_status);

//             let json_resp = response.json::<response::GenericResponse>().await.unwrap();
//             assert_eq!(json_resp.rows, Some(expected_rows));

//             println!(
//                 "\n=> Route: /items/add\n=> Added {} rows: {:?}\n",
//                 expected_rows, json_resp
//             );
//             // let _
//         }
//         Err(err) => {
//             eprintln!("\n=> Route: /items/add\n=> Unintended error: {}\n", err);
//             panic!("Failed to get add items response");
//         }
//     }
//     let _ = delete_table_by_id(999).await; // Cleanup
// }

// #[rstest]
// #[case(999, 1, 200)]
// #[case(999, 0, 200)]
// #[tokio::test]
// async fn test_delete_item_by_id(
//     #[case] table_id: u32,
//     #[case] rows_affected: u64,
//     #[case] expected_status: u16,
// ) {
//     let item_id = if rows_affected == 1 {
//         // Add an item for deletion
//         let _ = add_item(AddItemsRequest {
//             to_add: vec![TableItem {
//                 table_id: table_id,
//                 item: "Burger".to_string(),
//                 customer_id: Some("Bob".to_string()),
//             }],
//         })
//         .await;

//         // Fetch the id of the item that was just added
//         let x = get_items(GetItemRequest {
//             table_id: table_id,
//             item: Some("Burger".to_string()),
//             customer_id: Some("Bob".to_string()),
//         })
//         .await
//         .unwrap()
//         .json::<ItemsResponse>()
//         .await
//         .unwrap();
//         println!("@@@{:?}", x);
//         11
//         // .items[0]
//         //     .id
//     } else {
//         999
//     };

//     match delete_item_by_id(item_id).await {
//         Ok(response) => {
//             assert!(response.status().as_u16() == expected_status);

//             let json_resp = response.json::<GenericResponse>().await.unwrap();
//             assert_eq!(json_resp.rows, Some(rows_affected));

//             println!(
//                 "\n=> Route: /items/delete/{}\n=> Response for item {}: {:?}\n",
//                 item_id, item_id, json_resp
//             );
//         }

//         Err(err) => {
//             eprintln!(
//                 "\n=> Route: /items/delete/{}\n=> Unintended error: {}\n",
//                 item_id, err
//             );
//             panic!("Failed to get delete item response");
//         }
//     }
// }

// Helpers

async fn get_test_server() -> (Client, String) {
    // Need env vars for connecting to host
    dotenv().ok();
    let app_host = std::env::var("APP_HOST").expect("APP_HOST env var not set!");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT env var not set!");
    let addr = format!("http://{}:{}", app_host, app_port);
    println!("\n=> Host: {}\n", addr,);
    (Client::new(), addr)
}

async fn add_table(table_id: u32, seats: u32) -> Result<reqwest::Response, reqwest::Error> {
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

async fn delete_table_by_id(table_id: u32) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/table/delete/".to_string() + &table_id.to_string();

    client.delete(host + &route).send().await
}

async fn get_items(request: GetItemRequest) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/items".to_string();

    client.post(host + &route).json(&request).send().await
}

async fn add_item(request: AddItemsRequest) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/items/add".to_string();

    client.put(host + &route).json(&request).send().await
}

async fn delete_item_by_id(item_id: u32) -> Result<reqwest::Response, reqwest::Error> {
    let (client, host) = get_test_server().await;
    let route = "/items/delete/".to_string() + &item_id.to_string();

    client.delete(host + &route).send().await
}
