use super::mapper::*;
use chrono::{NaiveDateTime, Utc};
use sqlx::PgConnection;

pub async fn insert(
    conn: &mut PgConnection,
    new_user: &NewUser<'_>,
) -> crate::Result<(i64, NaiveDateTime)> {
    let created_at = Utc::now().naive_utc();

    struct Returning {
        id: i64,
    }

    let r = sqlx::query_as!(
        Returning,
        r#"
INSERT INTO users
    (
        username, first_name, last_name, 
        email, encrypted_password, 
        phone, status, 
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
        new_user.status,
        created_at,
        created_at,
    )
    .fetch_one(conn)
    .await?;

    Ok((r.id, created_at))
}
