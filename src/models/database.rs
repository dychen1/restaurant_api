use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Table {
    pub id: u32,
    pub seats: u32,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Items {
    pub id: u32,
    pub table_id: u32,
    pub item: String,
    pub cook_time: u8,
    pub customer_id: Option<String>,
    pub created_at: DateTime<Utc>,
}
