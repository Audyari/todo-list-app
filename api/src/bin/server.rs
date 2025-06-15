

use sqlx::MySqlPool;
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    // Initialize database pool
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    let pool = MySqlPool::connect(&database_url).await?;
    
    // Run migrations
    println!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("✅ Database connected and migrations run successfully!");
    println!("🎉 Setup database completed!");
    Ok(())
}