mod handlers;
mod logger;
mod services;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::{
    auth::{handle_login, handle_signup, serve_index, serve_signup},
    dashboard::serve_dashboard,
};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

use crate::handlers::{
    auth::handle_logout,
    dashboard::{delete_file, upload_file},
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port: u16 = args
        .get(1)
        .unwrap_or(&"80".to_string())
        .parse()
        .expect("Invalid port");

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/login", post(handle_login))
        .route("/signup", get(serve_signup))
        .route("/signup", post(handle_signup))
        .route("/logout", post(handle_logout))
        .route("/dashboard", get(serve_dashboard))
        .route("/upload-file", post(upload_file))
        .route("/delete_file", post(delete_file));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
