pub mod api;
pub mod redis;
pub mod model;
pub mod repository;
pub mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    api::start("0.0.0.0:8000").await?;
    Ok(())
}
