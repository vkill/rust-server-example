use super::mapper::*;
use chrono::Utc;
use sqlx::PgConnection;

pub async fn insert(conn: &mut PgConnection, user: &NewUser<'_>) -> crate::Result<User> {
    sqlx::query_as!(
        User,
        r#"
INSERT INTO users
    (
        username, first_name, last_name, 
        email, encrypted_password, 
        phone, user_status, 
        created_at, updated_at
    )
VALUES
    (
        $1, $2, $3,
        $4, $5,
        $6, $7,
        $8, $9
    )
RETURNING
    id,
    username, first_name, last_name, 
    email, encrypted_password, 
    phone, user_status, 
    created_at, updated_at
"#,
        user.username,
        user.first_name,
        user.last_name,
        user.email,
        user.encrypted_password,
        user.phone,
        user.user_status,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .fetch_one(conn)
    .await
}
