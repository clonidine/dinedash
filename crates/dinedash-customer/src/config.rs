pub mod db {
    use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};

    pub struct Credentials {
        pub user: String,
        pub password: String,
    }

    pub const SERVER_ADDRESS: &str = "postgres://customer-db:5432";

    pub async fn connect(
        cred: Credentials,
    ) -> std::io::Result<(Client, Connection<Socket, NoTlsStream>)> {
        Ok(tokio_postgres::connect(
            &format!(
                "host=localhost user={} password={}",
                cred.user, cred.password
            ),
            NoTls,
        )
        .await
        .unwrap())
    }
}

pub mod api {
    pub const SERVER_ADDRESS: &str = "0.0.0.0:9000";
}
