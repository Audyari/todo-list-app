use sqlx::mysql::MySqlPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diisi");
    let pool = MySqlPool::connect(&database_url).await?;
    
    // Cek versi MySQL
    let row: (String,) = sqlx::query_as("SELECT VERSION()")
        .fetch_one(&pool)
        .await?;
    println!("✅ Berhasil terhubung ke MySQL versi: {}", row.0);
    
    // Cek apakah tabel tasks ada
    let table_exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM information_schema.tables 
         WHERE table_schema = DATABASE() AND table_name = 'tasks'"
    )
    .fetch_one(&pool)
    .await?;
    
    if table_exists.0 > 0 {
        println!("✅ Tabel 'tasks' berhasil dibuat");
    } else {
        println!("❌ Tabel 'tasks' tidak ditemukan");
    }
    
    Ok(())
}