use dotenv::dotenv;
use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};

pub async fn connect() -> anyhow::Result<(Client, Connection<Socket, NoTlsStream>)> {
    dotenv().ok();

    let user = dotenv::var("POSTGRES_USER")?;
    let password = dotenv::var("POSTGRES_PASSWORD")?;

    Ok(tokio_postgres::connect(
        &format!("host=localhost user={user} password={password}"),
        NoTls,
    )
    .await?)
}
