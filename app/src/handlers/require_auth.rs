use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::CookieJar;

use crate::{
    handlers::helpers::{extract_credentials, remove_credentials},
    services::auth::{AuthResult, authenticate_user},
};

pub async fn require_auth(jar: CookieJar) -> Result<String, (CookieJar, Response<Body>)> {
    // Extract the credentials
    let (username, password) = extract_credentials(jar.clone());
    let (Some(username), Some(password)) = (username, password) else {
        return Err((remove_credentials(jar), Redirect::to("/").into_response()));
    };

    // Check the credentials
    let result = authenticate_user(username.clone(), password).await;
    match result {
        AuthResult::Ok => Ok(username),

        AuthResult::InvalidCredentials => {
            Err((remove_credentials(jar), Redirect::to("/").into_response()))
        }

        AuthResult::InternalError => Err((jar, StatusCode::INTERNAL_SERVER_ERROR.into_response())),
    }
}
