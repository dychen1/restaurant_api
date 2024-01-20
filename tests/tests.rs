use dotenv::dotenv;
use reqwest::blocking::Client;
use rstest::rstest;

#[path = "../src/models/response.rs"]
mod response;
use response::{GenericResponse, GetSeatsResponse, ItemsResponse};

#[path = "../src/models/request.rs"]
mod request;
use request::{AddItemsRequest, GetItemRequest, TableItem};

#[path = "../src/models/database.rs"]
mod database;

#[rstest]
fn test_health() {
    let (client, host) = get_test_server();
    let route = &"/health".to_string();

    match client.get(host + &route).send() {
        Ok(response) => {
            assert!(response.status() == 200);
            println!(
                "\n=> Route: {}\n=> Response: {}\n",
                route,
                response.text().unwrap()
            );
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            panic!("Failed to get health check response");
        }
    };
}

#[rstest]
#[case(1, 200, 4)] // Get table that exists, has 4 seats
#[case(999, 500, 0)] // Get table that doesn't exist
fn test_get_seats(
    #[case] table_id: u32,
    #[case] expected_status: u16,
    #[case] expected_seats: u32,
) {
    let (client, host) = get_test_server();
    let route = "/table/".to_string() + &table_id.to_string();

    match client.get(host + &route).send() {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            if expected_status == 200 {
                let json_resp = response.json::<GetSeatsResponse>().unwrap();
                assert_eq!(json_resp.seats, expected_seats);
                println!(
                    "\n=> Route: {}\n=> Response for table {}: {:?}\n",
                    route, table_id, json_resp
                );
            } else {
                println!(
                    "\n=> Route: {}\n=> Intended error response: {}\n",
                    route,
                    response.text().unwrap()
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
#[case(999, 1, 200)] // Add table that doesnt exist
#[case(999, 1, 500)] // Add table that already exists
fn test_add_table(#[case] table_id: u32, #[case] seats: u32, #[case] expected_status: u16) {
    match add_table(table_id, seats) {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            if expected_status == 200 {
                let json_resp = response.json::<response::GenericResponse>().unwrap();
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
                    response.text().unwrap()
                );
                let _ = delete_table_by_id(table_id); // Cleanup
            }
        }

        Err(err) => {
            eprintln!("\n=> Route: /table/add\n=> Unintended error: {}\n", err);
            panic!("Failed to get add table response");
        }
    };
}

#[rstest]
#[case(999, 1, 200)] // Delete table that exists
#[case(999, 0, 200)] // Delete table that doesn't exist
fn test_delete_table_by_id(
    #[case] table_id: u32,
    #[case] rows_affected: u64,
    #[case] expected_status: u16,
) {
    if rows_affected == 1 {
        let _ = add_table(table_id, 1); // Add table for deletion
    }
    match delete_table_by_id(table_id) {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<GenericResponse>().unwrap();

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
#[case(GetItemRequest{table_id: 1, item: None, customer_id: None}, 4, 200)] // Get all items for table 1
#[case(GetItemRequest{table_id: 1, item: Some("Bun Cha".to_string()), customer_id: None}, 2, 200)] // Get specific item for table 1
#[case(GetItemRequest{table_id: 1, item: Some("Bun Cha".to_string()), customer_id: Some("Anthony Bourdain".to_string())}, 1, 200)] // Get specific item for table 1 and customer
#[case(GetItemRequest{table_id: 999, item: None, customer_id: None}, 0, 200)] // Get items for table that doesn't exist
fn test_get_items(
    #[case] request: GetItemRequest,
    #[case] expected_rows: usize,
    #[case] expected_status: u16,
) {
    match get_items(request) {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<ItemsResponse>().unwrap();
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

#[rstest]
#[case(AddItemsRequest{to_add: vec![ TableItem{table_id: 999, item: "Burger".to_string(), customer_id: Some("Bob".to_string())}] } , 1, 200)]
fn test_add_item(
    #[case] request: AddItemsRequest,
    #[case] expected_rows: u64,
    #[case] expected_status: u16,
) {
    let _ = add_table(999, 1); // Add table for item

    match add_item(request) {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<response::GenericResponse>().unwrap();
            assert_eq!(json_resp.rows, Some(expected_rows));

            println!(
                "\n=> Route: /items/add\n=> Added {} rows: {:?}\n",
                expected_rows, json_resp
            );
        }
        Err(err) => {
            eprintln!("\n=> Route: /items/add\n=> Unintended error: {}\n", err);
            panic!("Failed to get add items response");
        }
    }
    let _ = delete_table_by_id(999); // Cleanup table and item associated with table
}

#[rstest]
#[case(999, 1, 200)] // Delete item that exists
#[case(999, 0, 200)] // Delete item that doesn't exist
fn test_delete_item_by_id(
    #[case] table_id: u32,
    #[case] rows_affected: u64,
    #[case] expected_status: u16,
) {
    let item_id = if rows_affected == 1 {
        // Add table for item
        let _ = add_table(table_id, 1);

        // Add an item for deletion
        let _ = add_item(AddItemsRequest {
            to_add: vec![TableItem {
                table_id: table_id,
                item: "Burger".to_string(),
                customer_id: Some("Bob".to_string()),
            }],
        });

        // Fetch the id of the item that was just added
        get_items(GetItemRequest {
            table_id: table_id,
            item: Some("Burger".to_string()),
            customer_id: Some("Bob".to_string()),
        })
        .unwrap()
        .json::<ItemsResponse>()
        .unwrap()
        .items[0]
            .id
    } else {
        999
    };

    match delete_item_by_id(item_id) {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<GenericResponse>().unwrap();
            assert_eq!(json_resp.rows, Some(rows_affected));

            println!(
                "\n=> Route: /items/delete/{}\n=> Response for item {}: {:?}\n",
                item_id, item_id, json_resp
            );
        }

        Err(err) => {
            eprintln!(
                "\n=> Route: /items/delete/{}\n=> Unintended error: {}\n",
                item_id, err
            );
            panic!("Failed to get delete item response");
        }
    }
}

#[rstest]
#[case(TableItem {
    table_id: 999,
    item: "Burger".to_string(),
    customer_id: Some("Bob".to_string()),
}, 1, 200)] // Delete item that exists
#[case(TableItem {
    table_id: 999,
    item: "Burger".to_string(),
    customer_id: Some("Bob".to_string()),
}, 0, 200)] // Delete item that doesn't exist
fn test_delete_item(
    #[case] item: TableItem,
    #[case] expected_rows: u64,
    #[case] expected_status: u16,
) {
    if expected_rows > 0 {
        // Add table for item
        let _ = add_table(item.table_id, 1);

        // Add an item for deletion
        let _ = add_item(AddItemsRequest {
            to_add: vec![TableItem {
                table_id: item.table_id,
                item: "Burger".to_string(),
                customer_id: Some("Bob".to_string()),
            }],
        });
    };
    let table_id = item.table_id; // Needed for cleanup

    match delete_item(item) {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            let json_resp = response.json::<GenericResponse>().unwrap();
            assert_eq!(json_resp.rows, Some(expected_rows));

            println!(
                "\n=> Route: /items/delete\n=> Response for delete item: {:?}\n",
                json_resp
            );
        }

        Err(err) => {
            eprintln!("\n=> Route: /items/delete\n=> Unintended error: {}\n", err);
            panic!("Failed to get delete item response");
        }
    }
    let _ = delete_table_by_id(table_id); // Cleanup table
}

// Helpers
type TestResponse = Result<reqwest::blocking::Response, reqwest::Error>;

fn get_test_server() -> (Client, String) {
    // Need env vars for connecting to host
    dotenv().ok();
    let app_host = std::env::var("APP_HOST").expect("APP_HOST env var not set!");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT env var not set!");
    let addr = format!("http://{}:{}", app_host, app_port);
    println!("\n=> Host: {}\n", addr,);
    (Client::new(), addr)
}

fn add_table(table_id: u32, seats: u32) -> TestResponse {
    let (client, host) = get_test_server();
    let route = "/table/add".to_string();

    client
        .put(host + &route)
        .json(&database::Table {
            id: table_id,
            seats: seats,
        })
        .send()
}

fn delete_table_by_id(table_id: u32) -> TestResponse {
    let (client, host) = get_test_server();
    let route = "/table/delete/".to_string() + &table_id.to_string();

    client.delete(host + &route).send()
}

fn get_items(request: GetItemRequest) -> TestResponse {
    let (client, host) = get_test_server();
    let route = "/items".to_string();

    client.post(host + &route).json(&request).send()
}

fn add_item(request: AddItemsRequest) -> TestResponse {
    let (client, host) = get_test_server();
    let route = "/items/add".to_string();

    client.put(host + &route).json(&request).send()
}

fn delete_item_by_id(item_id: u32) -> TestResponse {
    let (client, host) = get_test_server();
    let route = "/items/delete/".to_string() + &item_id.to_string();

    client.delete(host + &route).send()
}

fn delete_item(item: TableItem) -> TestResponse {
    let (client, host) = get_test_server();
    let route = "/items/delete".to_string();

    client.delete(host + &route).json(&item).send()
}
