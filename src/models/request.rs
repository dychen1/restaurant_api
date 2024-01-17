use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddItemRequest {
    pub table_id: u32,
    pub items: Vec<String>,
    pub customer_id: String,
}

#[derive(Deserialize, Debug)]
pub struct RemoveItemRequest {
    pub table_id: u32,
    pub items: String,
    pub customer_id: String,
}
