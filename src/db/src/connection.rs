use sqlx::{pool::PoolConnection, Database, Pool};

#[derive(Clone)]
pub struct Connection<DB>
where
    DB: Database,
{
    pool: Pool<DB::Connection>,
}

impl<DB> Connection<DB>
where
    DB: Database,
{
    pub async fn new(url: &str) -> crate::Result<Self> {
        let pool: Pool<DB::Connection> = Pool::new(url).await?;

        Ok(Self { pool })
    }

    pub async fn conn(&self) -> crate::Result<PoolConnection<DB::Connection>> {
        self.pool.acquire().await
    }
}
