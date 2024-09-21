use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use tokio_postgres::NoTls;
use dotenvy::dotenv;
use std::env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();

    if args.debug {
        // Debug print to check if environment variables are loaded
        for (key, value) in env::vars() {
            println!("{key}: {value}");
        }
    }

    let database_url = env
        ::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment or .env file");
    let redis_url = env
        ::var("REDIS_URL")
        .expect("REDIS_URL must be set in the environment or .env file");

    println!("Initializing PostgreSQL connection pool...");
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;
    let pg_pool = match Pool::builder().build(pg_mgr).await {
        Ok(pool) => {
            println!("PostgreSQL connection pool created successfully.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to create PostgreSQL connection pool: {}", e);
            return Err(e.into());
        }
    };

    println!("Initializing Redis connection pool...");
    let redis_mgr = RedisConnectionManager::new(redis_url)?;
    let redis_pool = match Pool::builder().build(redis_mgr).await {
        Ok(pool) => {
            println!("Redis connection pool created successfully.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to create Redis connection pool: {}", e);
            return Err(e.into());
        }
    };

    println!("Attempting to get a PostgreSQL connection from the pool...");
    let pg_client = match pg_pool.get().await {
        Ok(client) => {
            println!("Successfully acquired a PostgreSQL connection.");
            client
        }
        Err(e) => {
            eprintln!("Failed to get a PostgreSQL connection: {}", e);
            return Err(e.into());
        }
    };

    println!("Executing PostgreSQL query...");
    match pg_client.query("SELECT 1", &[]).await {
        Ok(rows) => println!("PostgreSQL query result: {:?}", rows),
        Err(e) => eprintln!("PostgreSQL query failed: {}", e),
    }

    println!("Attempting to get a Redis connection from the pool...");
    let mut redis_conn = match redis_pool.get().await {
        Ok(conn) => {
            println!("Successfully acquired a Redis connection.");
            conn
        }
        Err(e) => {
            eprintln!("Failed to get a Redis connection: {}", e);
            return Err(e.into());
        }
    };

    println!("Executing Redis commands...");
    match redis_conn.set::<_, _, ()>("key", "value").await {
        Ok(_) => println!("Successfully set Redis key."),
        Err(e) => eprintln!("Failed to set Redis key: {}", e),
    }

    match redis_conn.get::<_, String>("key").await {
        Ok(value) => println!("Redis query result: {}", value),
        Err(e) => eprintln!("Failed to get Redis value: {}", e),
    }

    Ok(())
}
