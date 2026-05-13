use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileMeta {
    pub name: String,
    pub size: u64,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub files: Vec<FileMeta>,
}
