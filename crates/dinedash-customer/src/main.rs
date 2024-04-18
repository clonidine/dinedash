use std::io;

pub mod api;
pub mod config;
pub mod repository;
pub mod routes;

#[tokio::main]
async fn main() -> io::Result<()> {
    api::start().await;
    Ok(())
}
