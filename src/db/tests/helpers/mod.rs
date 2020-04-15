pub async fn get_test_connection() -> db::Result<db::Connection> {
    db::Connection::new(&dotenv::var("DATABASE_URL").ok().expect("")).await
}
