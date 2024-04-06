use axum::{routing::get, Router};
use mobc::Pool;

use mobc_redis::{redis::Client, RedisConnectionManager};

use crate::redis;

const POOL_MAX_CONNECTIONS: u64 = 5;

/// Starts the server.
///
/// # Arguments
///
/// * `address` - The address to bind to.
///
/// # Returns
///
/// A `core::result::Result` containing the result of starting the server.
pub async fn start(address: &str) -> anyhow::Result<()> {
    let manager = RedisConnectionManager::new(Client::open(redis::SERVER_ADDR)?);

    let pool = Pool::builder()
        .max_open(POOL_MAX_CONNECTIONS)
        .build(manager);

    let app = Router::new()
        .route("/", get(|| async { "Hello!" }))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("Server running at {address}");

    axum::serve(listener, app).await?;

    Ok(())
}
