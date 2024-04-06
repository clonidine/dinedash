pub mod api;
pub mod redis;
pub mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    api::start("0.0.0.0:8000").await?;
    Ok(())
}
