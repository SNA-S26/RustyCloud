use mongodb::{bson::doc};
use redis::{AsyncCommands};
use std::{result::Result};
use crate::{logger::logger::error, storage::models::User, storage::database::{get_db, get_redis}};

pub async fn is_valid_authenticity(
    username: String,
    password_hash: String,
) -> Result<bool, String> {
    let cache_key = format!("user:auth:{}", username);

    // Connect to Redis
    let mut redis_conn = match get_redis().get_multiplexed_async_connection().await {
        Ok(conn) => Some(conn),

        Err(e) => {
            error(format!("Redis connection failed: {}", e).as_str()).await;
            None
        }
    };

    // Check Redis (cache)
    if let Some(conn) = redis_conn.as_mut() {
        let cached: Result<Option<String>, _> = conn.get(&cache_key).await;

        match cached {
            Ok(Some(cached)) => {
                if cached == password_hash {
                    return Ok(true);
                }
            }

            Ok(_) => {}

            Err(e) => error(format!("Redis GET failed: {}", e).as_str()).await, // not critical
        }
    }

    // Check Mongo
    let users = get_db().collection::<User>("users");

    let user = users
        .find_one(doc! {
            "username": &username
        })
        .await;

    let user: User = match user {
        Ok(Some(user)) => user,

        Ok(_) => return Ok(false),

        Err(e) => return Err(e.to_string()), // critical
    };

    // Cache the auth
    if let Some(conn) = redis_conn.as_mut() {
        let _: Result<(), _> = conn.set_ex(&cache_key, &user.password_hash, 3600).await;
    }

    Ok(user.password_hash == password_hash)
}

pub async fn try_register_user(
    username: String,
    password_hash: String,
) -> Result<bool, String> {
    let users = get_db().collection::<User>("users");

    // Check if the user already exists
    let user = users
        .find_one(doc! {
            "username": &username
        })
        .await
        .map_err(|e| e.to_string())?;

    if user.is_some() {
        return Ok(false);
    }

    // Register the user
    let user = User {
        username: username,
        password_hash: password_hash,
        files: Vec::new()
    };

    users.insert_one(user).await.map_err(|e| e.to_string())?;
    Ok(true)
}
