use sqlx::Postgres;

pub async fn get_postgres_connection() -> db::Result<db::Connection<Postgres>> {
    db::Connection::<Postgres>::new(&dotenv::var("DATABASE_URL").ok().expect("")).await
}
