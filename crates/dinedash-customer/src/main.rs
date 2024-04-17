use std::io;

pub mod api;
pub mod config;

#[tokio::main]
async fn main() -> io::Result<()> {
    api::start().await;
    Ok(())
}
