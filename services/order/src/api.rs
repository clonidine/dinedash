use axum::{routing::get, Router};
use tokio::net::TcpListener;

/// Starts the server.
///
/// # Arguments
///
/// * `address` - The address to bind to.
///
/// # Returns
///
/// A `std::io::Result` containing the result of starting the server.
pub async fn start(address: &str) -> std::io::Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello!" }));

    let listener = TcpListener::bind(address).await?;

    println!("Server running at {address}");

    axum::serve(listener, app).await?;

    Ok(())
}
