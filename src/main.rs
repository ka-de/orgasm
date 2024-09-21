use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use tokio_postgres::NoTls;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pg_mgr = PostgresConnectionManager::new_from_stringlike(
        env::var("DATABASE_URL").unwrap(),
        NoTls
    ).unwrap();
    let pg_pool = Pool::builder().build(pg_mgr).await.unwrap();

    let redis_mgr = RedisConnectionManager::new(env::var("REDIS_URL").unwrap()).unwrap();
    let redis_pool = Pool::builder().build(redis_mgr).await.unwrap();

    let pg_client = pg_pool.get().await.unwrap();
    let rows = pg_client.query("SELECT 1", &[]).await.unwrap();
    println!("PostgreSQL query result: {:?}", rows);

    let mut redis_conn = redis_pool.get().await.unwrap();
    let _: () = redis_conn.set("key", "value").await.unwrap();
    let value: String = redis_conn.get("key").await.unwrap();
    println!("Redis query result: {}", value);
}
