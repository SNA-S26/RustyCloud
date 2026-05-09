use axum::{Form, Json, Router, response::Html, routing::{get, post}};
use std::{env, net::SocketAddr};
use tokio::{fs, net::TcpListener};
use serde::{Deserialize, Serialize};

/*
 * FRONTEND
 * Serving static pages:
 * - Index page
 * - Sign up page
 */

async fn serve_index() -> Html<String> {
    let index = fs::read_to_string("static/index.html").await.unwrap();
    Html(index)
}

async fn serve_signup() -> Html<String> {
    let index = fs::read_to_string("static/signup.html").await.unwrap();
    Html(index)
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

async fn handle_login(Form(form): Form<AuthRequest>) -> Json<AuthRequest> {
    Json(form)
}

async fn handle_signup(Form(form): Form<AuthRequest>) -> Json<AuthRequest> {
    Json(form)
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
