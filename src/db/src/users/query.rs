use super::mapper::*;
use chrono::{NaiveDateTime, Utc};
use sqlx::PgConnection;

struct InsertReturning {
    id: i64,
}

pub async fn insert(
    conn: &mut PgConnection,
    new_user: &NewUser<'_>,
) -> crate::Result<(i64, NaiveDateTime)> {
    let created_at = Utc::now().naive_utc();

    let r = sqlx::query_as!(
        InsertReturning,
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
    id
"#,
        new_user.username,
        new_user.first_name,
        new_user.last_name,
        new_user.email,
        new_user.encrypted_password,
        new_user.phone,
        new_user.user_status,
        created_at,
        created_at,
    )
    .fetch_one(conn)
    .await?;

    Ok((r.id, created_at))
}
