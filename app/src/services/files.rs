use crate::logger::logger::{error, info};
use serde::Serialize;

#[derive(Serialize)]
pub struct FileMeta {
    name: String,
    size: u64,
    created_at: u64,
}

pub async fn get_files(username: String) -> Vec<FileMeta> {
    vec![
        FileMeta {
            name: "first.png".to_string(),
            size: 1_000,
            created_at: 1,
        },
        FileMeta {
            name: "second.jpg".to_string(),
            size: 1_000_000,
            created_at: 2,
        },
        FileMeta {
            name: "third.svg".to_string(),
            size: 64,
            created_at: 3,
        },
    ]
}

pub async fn write_file(username: String, filename: String) {
    info(format!("Writing file {} for user {}", filename, username).as_str()).await;
}

pub async fn delete_file(username: String, filename: String) {
    info(format!("Deleting file {} for user {}", filename, username).as_str()).await;
}
