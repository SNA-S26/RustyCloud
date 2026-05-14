use crate::logger::logger::{error, info, warning};
use crate::storage::files::{retrieve_file, retrieve_files, store_file};
use crate::storage::models::FileMeta;
use std::path::{Component, Path};

pub async fn get_files(username: String) -> Vec<FileMeta> {
    match retrieve_files(&username).await {
        Ok(files) => {
            info(format!("Retrieved files for user {}", username).as_str()).await;
            files
        }

        Err(e) => {
            error(format!("Get files error: {}", e).as_str()).await;
            vec![]
        }
    }
}

fn sanitize_filename(filename: &str) -> Option<String> {
    let path = Path::new(filename);

    // Check for the absolute path or home shortcut
    if path.is_absolute() || path.starts_with("~") {
        return None;
    }

    // Check for the path traversal
    let mut components = path.components();
    match components.next() {
        Some(Component::Normal(_)) => {
            if components.next().is_some() {
                return None;
            }

            Some(filename.to_string())
        }

        _ => None,
    }
}

fn compose_path(username: &str, filename: &str) -> String {
    let storage = std::env::var("NFS_MOUNT_POINT").unwrap_or_else(|_| "/data".to_string());
    format!("{}/{}/{}", storage, username, filename)
}

pub async fn get_file(username: &str, filename: &str) -> Result<Vec<u8>, ()> {
    let Some(filename) = sanitize_filename(filename) else {
        warning(format!("Invalid file {} provided by user {}", filename, username).as_str()).await;
        return Err(());
    };

    info(format!("Writing file {} for user {}", filename, username).as_str()).await;
    let path = compose_path(&username, &filename);

    match retrieve_file(&path).await {
        Ok(file) => {
            info(format!("Retrieved file {} for user {}", filename, username).as_str()).await;
            Ok(file)
        }

        Err(e) => {
            error(format!("Get file error: {}", e).as_str()).await;
            Err(())
        }
    }
}

pub async fn write_file(username: String, filename: String, data: impl AsRef<[u8]>) {
    let Some(filename) = sanitize_filename(filename.as_str()) else {
        warning(format!("Invalid file {} provided by user {}", filename, username).as_str()).await;
        return;
    };

    info(format!("Writing file {} for user {}", filename, username).as_str()).await;
    let path = compose_path(&username, &filename);
    match store_file(&username, &filename, &path, data).await {
        Ok(_) => {}

        Err(e) => {
            error(format!("Write file error: {}", e).as_str()).await;
        }
    };
}

pub async fn remove_file(username: String, filename: String) {
    let Some(filename) = sanitize_filename(filename.as_str()) else {
        warning(format!("Invalid file {} provided by user {}", filename, username).as_str()).await;
        return;
    };

    info(format!("Removing file {} for user {}", filename, username).as_str()).await;
    let path = compose_path(&username, &filename);
    match crate::storage::files::remove_file(&username, &filename, &path).await {
        Ok(_) => {}

        Err(e) => {
            error(format!("Remove file error: {}", e).as_str()).await;
        }
    };
}
