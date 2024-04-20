use axum::{routing::get, Router};
use dotenv::dotenv;

use crate::{config, db};

pub async fn start() -> anyhow::Result<()> {
    dotenv().ok();

    let _pool = db::connect().await?;

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(config::api::SERVER_ADDRESS).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
