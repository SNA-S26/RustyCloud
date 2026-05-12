mod handlers;
mod logger;
mod services;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::auth::{handle_login, handle_signup, serve_index, serve_signup};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

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
        .route("/signup", post(handle_signup));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
