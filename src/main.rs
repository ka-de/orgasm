//! Orgasm Imageboard Backend Initialization
//!
//! This script initializes and tests connections to PostgreSQL and Redis databases
//! for the Orgasm imageboard (booru) application. It sets up connection pools for both
//! databases, performs basic queries to ensure connectivity, and provides debug output
//! options for troubleshooting.
//!
//! The script uses environment variables for database configuration and supports
//! command-line arguments for additional debugging information.

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use tokio_postgres::NoTls;
use dotenvy::dotenv;
use std::env;
use clap::Parser;
use argon2::{ self, Config };
use rand::Rng;

/// Command-line arguments for the application
///
/// This struct defines the command-line arguments that can be passed to the application.
/// It uses the `clap` crate for parsing and handling these arguments.
///
/// # Fields
///
/// * `debug` - A boolean flag that enables debug output when set to true.
///             This can be useful for troubleshooting and viewing additional information
///             during runtime.
///
/// # Usage
///
/// To use debug mode, run the application with the `--debug` flag:
/// ```
/// $ ./orgasm --debug
/// ```
///
/// Or with the short form:
/// ```
/// $ ./orgasm -d
/// ```
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
        print_env_vars();
    }

    let database_url = get_env_var("DATABASE_URL");
    let redis_url = get_env_var("REDIS_URL");

    let pg_pool = create_postgres_pool(&database_url).await?;
    let redis_pool = create_redis_pool(&redis_url).await?;

    test_postgres_connection(&pg_pool).await?;
    test_redis_connection(&redis_pool).await?;

    // Add this line to create the users table if it doesn't exist
    create_users_table(&pg_pool).await?;

    // Example usage of user registration
    let username = "example_user";
    let password = "secure_password123";
    match register_user(&pg_pool, username, password).await {
        Ok(_) => println!("User registered successfully!"),
        Err(e) => eprintln!("Failed to register user: {}", e),
    }

    Ok(())
}

fn print_env_vars() {
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}

fn get_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} must be set in the environment or .env file", key))
}

async fn create_postgres_pool(
    database_url: &str
) -> Result<Pool<PostgresConnectionManager<NoTls>>, Box<dyn std::error::Error>> {
    println!("Initializing PostgreSQL connection pool...");
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;
    Pool::builder()
        .build(pg_mgr).await
        .map_err(|e| {
            eprintln!("Failed to create PostgreSQL connection pool: {}", e);
            e.into()
        })
}

async fn create_redis_pool(
    redis_url: &str
) -> Result<Pool<RedisConnectionManager>, Box<dyn std::error::Error>> {
    println!("Initializing Redis connection pool...");
    let redis_mgr = RedisConnectionManager::new(redis_url)?;
    Pool::builder()
        .build(redis_mgr).await
        .map_err(|e| {
            eprintln!("Failed to create Redis connection pool: {}", e);
            e.into()
        })
}

async fn test_postgres_connection(
    pool: &Pool<PostgresConnectionManager<NoTls>>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to get a PostgreSQL connection from the pool...");
    let client = pool.get().await.map_err(|e| {
        eprintln!("Failed to get a PostgreSQL connection: {}", e);
        e
    })?;

    println!("Executing PostgreSQL query...");
    match client.query("SELECT 1", &[]).await {
        Ok(rows) => {
            println!("PostgreSQL query result: {:?}", rows);
            println!("Successfully connected to PostgreSQL!");
        }
        Err(e) => {
            eprintln!("PostgreSQL query failed: {}", e);
            eprintln!("Failed to connect to PostgreSQL.");
        }
    }

    Ok(())
}

async fn test_redis_connection(
    pool: &Pool<RedisConnectionManager>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to get a Redis connection from the pool...");
    let mut conn = pool.get().await.map_err(|e| {
        eprintln!("Failed to get a Redis connection: {}", e);
        e
    })?;

    println!("Executing Redis commands...");
    conn.set::<_, _, ()>("key", "value").await.map_err(|e| {
        eprintln!("Failed to set Redis key: {}", e);
        e
    })?;

    match conn.get::<_, String>("key").await {
        Ok(value) => {
            println!("Redis query result: {}", value);
            println!("Successfully connected to Redis!");
        }
        Err(e) => {
            eprintln!("Failed to get Redis value: {}", e);
            eprintln!("Failed to connect to Redis.");
        }
    }

    Ok(())
}

async fn create_users_table(
    pool: &Pool<PostgresConnectionManager<NoTls>>
) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )",
        &[]
    ).await?;
    Ok(())
}

async fn register_user(
    pool: &Pool<PostgresConnectionManager<NoTls>>,
    username: &str,
    password: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    // Generate a random salt
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    // Hash the password
    let password_hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;

    // Insert the new user into the database
    client.execute(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        &[&username, &password_hash]
    ).await?;

    Ok(())
}
