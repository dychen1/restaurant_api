use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

pub struct AppDatabase {
    pub connection_pool: MySqlPool,
}

pub async fn database_connect() -> Result<AppDatabase, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env var is not set!");
    //Need URL encoding for a real database
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!("Successfully connected to MySQL database!");
    Ok(AppDatabase {
        connection_pool: pool,
    })
}
