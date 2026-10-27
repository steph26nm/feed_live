use axum::{
    Router,
    extract::Json,
    routing::{get, post},
    serve::Listener,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize, Serialize)]
struct MessagePayload {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/echo", post(echo_json));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port: {addr}");

    println!("Server running at http://127.0.0.1:8000");

    axum::serve(listener, app).await.expect("Server failed");
}

async fn home() -> &'static str {
    "Hello from the server"
}

async fn echo_json(Json(payload): Json<MessagePayload>) -> Json<MessagePayload> {
    Json(payload)
}
