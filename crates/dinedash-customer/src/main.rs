use std::io;

pub mod api;
pub mod config;
pub mod db;
pub mod repository;
pub mod routes;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Server running at {}", config::api::SERVER_ADDRESS);
    api::start().await;
    Ok(())
}
