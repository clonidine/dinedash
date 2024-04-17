use axum::{routing::get, Router};

use crate::config;

pub async fn start() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(config::api::SERVER_ADDRESS)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
