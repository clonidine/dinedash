use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub mod model;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0";
    let port = "8000";

    let app = Router::new().route("/", get(|| async { "Hello!" }));

    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    println!("Server running at {address}:{port}");

    axum::serve(listener, app).await?;

    Ok(())
}
