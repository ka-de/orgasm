use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    // Set up PostgreSQL connection pool
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=postgres password=secret dbname=imageboard",
        NoTls
    ).unwrap();
    let pg_pool = Pool::builder().build(pg_mgr).await.unwrap();

    // Set up Redis connection pool
    let redis_mgr = RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    let redis_pool = Pool::builder().build(redis_mgr).await.unwrap();

    // Example usage of PostgreSQL pool
    let pg_client = pg_pool.get().await.unwrap();
    let rows = pg_client.query("SELECT 1", &[]).await.unwrap();
    println!("PostgreSQL query result: {:?}", rows);

    // Example usage of Redis pool
    let mut redis_conn = redis_pool.get().await.unwrap();
    let _: () = redis_conn.set("key", "value").await.unwrap();
    let value: String = redis_conn.get("key").await.unwrap();
    println!("Redis query result: {}", value);
}
