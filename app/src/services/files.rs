use crate::logger::logger::{info};
use crate::storage::models::FileMeta;

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

pub async fn write_file(username: String, filename: String, data: impl AsRef<[u8]>) {
    info(format!("Writing file {} for user {}", filename, username).as_str()).await;
}

pub async fn remove_file(username: String, filename: String) {
    info(format!("Removing file {} for user {}", filename, username).as_str()).await;
}
