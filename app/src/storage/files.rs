use crate::{
    logger::logger::error,
    storage::{
        database::get_db,
        models::{FileMeta, User},
    },
};
use chrono::Utc;
use mongodb::bson::doc;
use std::{path::Path, result::Result};
use tokio::fs::{self, create_dir_all};

pub async fn retrieve_file(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).await.map_err(|e| e.to_string())
}

pub async fn retrieve_files(username: &str) -> Result<Vec<FileMeta>, String> {
    let users = get_db().collection::<User>("users");
    let user = users
        .find_one(doc! {
            "username": username
        })
        .await
        .map_err(|e| e.to_string())?;

    match user {
        Some(u) => Ok(u.files),
        _ => {
            error(format!("Retrieve files error: no user {} was found", username).as_str()).await;
            Ok(vec![])
        }
    }
}

pub async fn store_file(
    username: &str,
    filename: &str,
    path: &str,
    data: impl AsRef<[u8]>,
) -> Result<(), String> {
    // Create the directory
    if let Some(dir) = Path::new(path).parent() {
        if let Err(e) = create_dir_all(dir).await {
            return Err(e.to_string());
        }
    } else {
        return Err("Invalid parent directory".to_string());
    }

    // Write the file
    fs::write(path, data.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    // Store the file in Mongo
    let meta = FileMeta {
        name: filename.to_string(),
        size: data.as_ref().len() as u64,
        created_at: Utc::now().to_rfc3339(),
    };

    let users = get_db().collection::<User>("users");

    let result = users
        .update_one(
            doc! { "username": username },
            doc! {
                "$push": {
                    "files": mongodb::bson::to_bson(&meta).unwrap()
                }
            },
        )
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn remove_file(username: &str, filename: &str, path: &str) -> Result<(), String> {
    // Remove the file from FS
    fs::remove_file(path).await.map_err(|e| e.to_string())?;

    // Remove the file from Mongo
    let users = get_db().collection::<User>("users");

    let result = users
        .update_one(
            doc! { "username": username },
            doc! {
                "$pull": {
                    "files": {
                        "name": filename
                    }
                }
            },
        )
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
