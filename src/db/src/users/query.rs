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

pub async fn find_by_email(conn: &mut PgConnection, email: &str) -> crate::Result<User> {
    sqlx::query_as!(User, r#"SELECT * FROM users where email = $1"#, email)
        .fetch_one(conn)
        .await
}

pub async fn find_by_id(conn: &mut PgConnection, id: i64) -> crate::Result<User> {
    sqlx::query_as!(User, r#"SELECT * FROM users where id = $1"#, id)
        .fetch_one(conn)
        .await
}

pub async fn update(
    conn: &mut PgConnection,
    id: i64,
    update_user: &UpdateUser<'_>,
) -> crate::Result<bool> {
    let n = sqlx::query!(
        r#"
UPDATE users 
SET
    username = $1,
    first_name = $2,
    last_name = $3,
    phone = $4
where id = $5
"#,
        update_user.username,
        update_user.first_name,
        update_user.last_name,
        update_user.phone,
        id,
    )
    .execute(conn)
    .await?;

    Ok(n == 1)
}
