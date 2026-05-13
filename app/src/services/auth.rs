use crate::{
    logger::logger::{error, info, warning},
    storage::auth::{is_valid_authenticity, try_register_user},
};

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

pub enum AuthResult {
    Ok,
    InvalidCredentials,
    InternalError,
}

pub async fn authenticate_user(username: String, password: String) -> AuthResult {
    let hash = hash_password(password);

    match is_valid_authenticity(username.clone(), hash).await {
        Ok(valid) => {
            if valid {
                info(format!("Authenticated user {}", username).as_str()).await;
                AuthResult::Ok
            } else {
                warning(format!("Invalid authentication attempt for user {}", username).as_str())
                    .await;
                AuthResult::InvalidCredentials
            }
        }

        Err(e) => {
            error(format!("Authenticate user error: {}", e).as_str()).await;
            AuthResult::InternalError
        }
    }
}

pub enum RegisterResult {
    Ok,
    UserExists,
    InternalError,
}

pub async fn register_user(username: String, password: String) -> RegisterResult {
    let hash = hash_password(password);
    match try_register_user(username.clone(), hash).await {
        Ok(registered) => {
            if registered {
                info(format!("Registered user {}", username).as_str()).await;
                RegisterResult::Ok
            } else {
                RegisterResult::UserExists
            }
        }

        Err(e) => {
            error(format!("Register user error {}", e).as_str()).await;
            RegisterResult::InternalError
        }
    }
}
