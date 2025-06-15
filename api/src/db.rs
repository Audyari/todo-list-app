use sqlx::mysql::MySqlPool;
use std::env;

pub type DbPool = MySqlPool;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    MySqlPool::connect(&database_url).await
}