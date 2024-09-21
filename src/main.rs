use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use tokio_postgres::NoTls;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Debug print to check if environment variables are loaded
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    let database_url = env
        ::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment or .env file");
    let redis_url = env
        ::var("REDIS_URL")
        .expect("REDIS_URL must be set in the environment or .env file");

    let pg_mgr = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;
    let pg_pool = Pool::builder().build(pg_mgr).await?;

    let redis_mgr = RedisConnectionManager::new(redis_url)?;
    let redis_pool = Pool::builder().build(redis_mgr).await?;

    let pg_client = pg_pool.get().await.unwrap();
    let rows = pg_client.query("SELECT 1", &[]).await.unwrap();
    println!("PostgreSQL query result: {:?}", rows);

    let mut redis_conn = redis_pool.get().await.unwrap();
    let _: () = redis_conn.set("key", "value").await.unwrap();
    let value: String = redis_conn.get("key").await.unwrap();
    println!("Redis query result: {}", value);

    Ok(())
}
