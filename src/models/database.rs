use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Table {
    pub id: u32,
    pub seats: u32,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct OrderItems {
    pub id: u32,
    pub table_id: u32,
    pub item: String,
    pub cook_time: u8,
    pub customer_id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}
