pub mod api;
pub mod model;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    api::start("0.0.0.0:8000").await?;
    Ok(())
}
