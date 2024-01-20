use rstest::rstest;

#[path = "../src/models/response.rs"]
mod response;
use response::{GenericResponse, GetSeatsResponse, ItemsResponse};

#[path = "../src/models/request.rs"]
mod request;
use request::GetItemRequest;

#[path = "../src/models/database.rs"]
mod database;

mod utils;
use crate::utils::{add_table, delete_table_by_id, get_test_server};

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
#[case(10000, 1, 200)]
#[case(10000, 0, 200)]
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
    #[case] get_item_request: GetItemRequest,
    #[case] expected_rows: usize,
    #[case] expected_status: u16,
) {
    let (client, host) = get_test_server().await;
    let route = "/items".to_string();

    match client
        .post(host + &route)
        .json(&get_item_request)
        .send()
        .await
    {
        Ok(response) => {
            assert!(response.status().as_u16() == expected_status);

            if expected_status == 200 {
                let json_resp = response.json::<response::ItemsResponse>().await.unwrap();
                assert_eq!(json_resp.items.len(), expected_rows);

                println!(
                    "\n=> Route: {}\n=> Response for table {}: {:?}\n",
                    route, get_item_request.table_id, json_resp
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
            eprintln!("\n=> Route: {}\n=> Unintended error: {}\n", route, err);
            panic!("Failed to get add items response");
        }
    }
}
