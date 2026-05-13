use super::helpers::{extract_cookie, render_error};
use crate::{
    handlers::helpers::{add_credentials, extract_credentials, remove_cookie, remove_credentials},
    services::auth::{AuthResult, RegisterResult, authenticate_user, register_user},
};
use axum::{
    Form,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};

/*
 * Handles GET /
 *
 * Sends index.html template for rendering login error,
 * returns the result HTML and cookies
 */
pub async fn serve_index(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    let (Some(_), Some(_)) = extract_credentials(jar.clone()) else {
        let description = extract_cookie(jar.clone(), "login_error");
        let jar = remove_cookie(jar, "login_error");
        return (
            jar,
            render_error("index.html", description)
                .await
                .into_response(),
        );
    };

    (jar, Redirect::to("/dashboard").into_response())
}

/*
 * Handles GET /signup
 *
 * Sends signup.html for rendering signup error,
 * returns the result HTML and cookies
 */
pub async fn serve_signup(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    let (Some(_), Some(_)) = extract_credentials(jar.clone()) else {
        let description = extract_cookie(jar.clone(), "signup_error");
        let jar = remove_cookie(jar, "signup_error");
        return (
            jar,
            render_error("signup.html", description)
                .await
                .into_response(),
        );
    };

    (jar, Redirect::to("/dashboard").into_response())
}

// Represents authentication request
// issued by a web client
#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

/*
 * Handles POST /login
 *
 * Extracts username and password
 * Passes further for a check:
 *  - Success:  Redirect to /dashboard
 *  - Fail:     Redirect to / with "login_error" cookie set
 */
pub async fn handle_login(
    jar: CookieJar,
    Form(form): Form<AuthRequest>,
) -> (CookieJar, impl IntoResponse) {
    let result = authenticate_user(form.username.clone(), form.password.clone()).await;

    match result {
        AuthResult::Ok => {
            let jar = add_credentials(jar, form.username, form.password);
            (jar, Redirect::to("/dashboard").into_response())
        }

        AuthResult::InvalidCredentials => {
            // Set the cookie and redirect
            let cookie = Cookie::build(("login_error", "Invalid username or password"))
                .path("/")
                .build();

            (jar.add(cookie), Redirect::to("/").into_response())
        }

        AuthResult::InternalError => {
            let cookie = Cookie::build(("login_error", "Internal Server Error"))
                .path("/")
                .build();

            (jar.add(cookie), Redirect::to("/").into_response())
        }
    }
}

/*
 * Handles POST /signup
 *
 * Extracts username and password
 * Passes further for a check:
 *  - Success:  Redirect to /dashboard
 *  - Fail:     Redirect to / with "signup_error" cookie set
 */
pub async fn handle_signup(
    jar: CookieJar,
    Form(form): Form<AuthRequest>,
) -> (CookieJar, impl IntoResponse) {
    let result = register_user(form.username.clone(), form.password.clone()).await;

    match result {
        RegisterResult::Ok => {
            let jar = add_credentials(jar, form.username, form.password);
            (jar, Redirect::to("/dashboard").into_response())
        }

        RegisterResult::UserExists => {
            // Set the cookie and redirect
            let cookie = Cookie::build(("login_error", "Username taken"))
                .path("/")
                .build();

            (jar.add(cookie), Redirect::to("/").into_response())
        }

        RegisterResult::InternalError => {
            let cookie = Cookie::build(("signup_error", "Internal Server Error"))
                .path("/")
                .build();

            (jar.add(cookie), Redirect::to("/signup").into_response())
        }
    }
}

/*
 * Handles POST /logout
 */
pub async fn handle_logout(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    (remove_credentials(jar), Redirect::to("/").into_response())
}
