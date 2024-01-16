use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

pub struct AppDatabase {
    db_pool: MySqlPool,
}

pub async fn database_connect() -> Result<AppDatabase, sqlx::Error> {
    // Load env vars from .env file
    dotenv().ok();

    let mysql_user: String = env::var("MYSQL_USER").expect("MYSQL_USER env var is not set!");
    let mysql_password: String =
        env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD env var is not set!");
    let mysql_database: String =
        env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE env var is not set!");
    let database_host: String =
        env::var("DATABASE_HOST").expect("DATABASE_HOST env var is not set!");
    let database_port: String =
        env::var("DATABASE_PORT").expect("DATABASE_PORT env var is not set!");

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_user, mysql_password, database_host, database_port, mysql_database
    );

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!("Successfully connected to MySQL database!");
    Ok(AppDatabase { db_pool: pool })
}
