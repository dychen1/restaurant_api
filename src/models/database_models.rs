use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Table {
    pub id: u32,
    pub seats: u32,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Orders {
    pub id: u32,
    pub table_id: u32,
    pub customer_id: Option<String>,
    pub is_active: i8,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct OrderItems {
    pub order_id: u32,
    pub item: String,
    pub number_of_items: u16,
    pub cook_time: u8,
    pub created_at: DateTime<Utc>,
}
