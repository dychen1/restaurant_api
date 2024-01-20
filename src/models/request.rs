use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct GetItemRequest {
    pub table_id: u32,
    pub item: Option<String>,
    pub customer_id: Option<String>,
}

// Used for adding and deleting items
#[derive(Deserialize, Debug, Clone)]
pub struct TableItem {
    pub table_id: u32,
    pub item: String,
    pub customer_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddItemsRequest {
    pub to_add: Vec<TableItem>,
}
