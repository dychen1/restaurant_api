use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ItemsRequest {
    pub table_id: u32,
    pub items: Vec<String>,
    pub customer_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ItemFields {
    pub table_id: u32,
    pub item: String,
    pub customer_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddItemsRequest {
    pub to_add: Vec<ItemFields>,
}
