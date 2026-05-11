use axum::{Form, Router, response::{Html, IntoResponse, Redirect}, routing::{get, post}};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::{env, net::SocketAddr};
use tokio::{net::TcpListener};
use serde::{Deserialize, Serialize};

/*
 * FRONTEND
 * Serving static pages:
 * - Index page
 * - Sign up page
 */

async fn render_error(path: &str, err: Option<String>) -> Html<String> {
    let mut ctx = tera::Context::new();

    if let Some(msg) = err {
        ctx.insert("error", &msg);
    }

    let html = tera::Tera::new("templates/**/*.html").unwrap()
        .render(path, &ctx).unwrap();

    Html(html)
}

// Extracts and REMOVES the cookie from jar
fn extract_cookie(jar: CookieJar, name: &str) -> (CookieJar, Option<String>) {
    let value = jar.get(name).map(|c| c.value().to_string());

    let jar = jar.remove(Cookie::from(name.to_string()));
    (jar, value)
}

async fn serve_index(jar: CookieJar) -> (CookieJar, Html<String>) {
    let (jar, value) = extract_cookie(jar, "login_error");
    (jar, render_error("index.html", value).await)
}

async fn serve_signup(jar: CookieJar) -> (CookieJar, Html<String>) {
    let (jar, value) = extract_cookie(jar, "signup_error");
    (jar, render_error("signup.html", value).await)
}

/*
 * AUTHORIZATION / SIGN UP
 * Stubs
 */

#[derive(Deserialize, Serialize)]
struct AuthRequest {
    username: String,
    password: String
}

async fn handle_login(jar: CookieJar, Form(form): Form<AuthRequest>) -> (CookieJar, impl IntoResponse) {
    if form.username == "Username" && form.password == "Password" {
        (jar, Html("Success").into_response())
    } else {
        let cookie = Cookie::build(("login_error", "Invalid username or password"))
            .path("/")
            .build();

        (jar.add(cookie), Redirect::to("/").into_response())
    }
}

async fn handle_signup(jar: CookieJar, Form(form): Form<AuthRequest>) -> (CookieJar, impl IntoResponse) {
    if form.username != "Username" {
        (jar, Html("Success").into_response())
    } else {
        let cookie = Cookie::build(("signup_error", "Username taken"))
            .path("/signup")
            .build();

        (jar.add(cookie), Redirect::to("/signup").into_response())
    }
}

/*
 * FILE STORING
 * TODO
 */


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port: u16 = args.get(1)
        .unwrap_or(&"80".to_string())
        .parse()
        .expect("Invalid port");

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/login", post(handle_login))
        .route("/signup", get(serve_signup))
        .route("/signup", post(handle_signup));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
