use mongodb::{Client as MongoClient, Database};
use redis::{Client as RedisClient};
use std::{env, result::Result, sync::OnceLock};

pub static MONGO_DB: OnceLock<Database> = OnceLock::new();
pub static REDIS_CLIENT: OnceLock<RedisClient> = OnceLock::new();

pub fn get_db() -> &'static Database {
    MONGO_DB.get().expect("MongoDB is not initialized")
}

pub fn get_redis() -> &'static RedisClient {
    REDIS_CLIENT.get().expect("Redis is not initialized")
}

pub async fn init_connections() -> Result<(), String> {
    // Initialize MongoDB
    let mongo_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://mongodb:27017".to_string());
    let mongo_client: MongoClient = MongoClient::with_uri_str(&mongo_uri)
        .await
        .map_err(|e| e.to_string())?;
    let db = mongo_client.database("appdb");
    MONGO_DB
        .set(db)
        .map_err(|_| "Failed to initialize global MongoDB variable")?;

    // Initialize Redis
    let redis_uri = env::var("REDIS_URI").unwrap_or_else(|_| "redis://redis:6379".to_string());
    let redis_client: RedisClient = RedisClient::open(redis_uri).map_err(|e| e.to_string())?;
    REDIS_CLIENT
        .set(redis_client)
        .map_err(|_| "Failed to initialize global Redis variable")?;

    Ok(())
}
