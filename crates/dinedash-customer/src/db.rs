use dotenv::dotenv;

pub async fn connect() -> anyhow::Result<sqlx::PgPool> {
    dotenv().ok();

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL var should be set");

    let pool = sqlx::PgPool::connect(&db_url).await?;

    Ok(pool)
}
