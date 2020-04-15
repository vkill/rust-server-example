use sqlx::PgPool;

#[derive(Clone)]
pub struct Connection {
    pub pool: PgPool,
}

impl Connection {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::new(url).await?;

        Ok(Self { pool })
    }
}
