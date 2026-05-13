use crate::logger::logger::{error, info, warning};

pub enum AuthResult {
    Ok,
    InvalidCredentials,
    InternalError,
}

pub enum RegisterResult {
    Ok,
    UserExists,
    InternalError,
}

/*
 * Produces a password hash using SHA256
 * Purpose: not to store passwords in the database
 */
fn hash_password(password: String) -> String {
    use sha2::{Digest, Sha256};

    let mut sha256 = Sha256::new();
    sha256.update(password);
    hex::encode(sha256.finalize())
}

pub async fn authenticate_user(username: String, password: String) -> AuthResult {
    let hash = hash_password(password);

    info(format!("Authenticated user {}", username).as_str()).await;
    AuthResult::Ok
}

pub async fn register_user(username: String, password: String) -> RegisterResult {
    let hash = hash_password(password);

    info(format!("Registered user {}", username).as_str()).await;
    RegisterResult::Ok
}
