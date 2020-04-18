mod helpers;
use helpers::get_postgres_connection;

use db::users::mapper::*;
use db::users::query::*;

use chrono::Utc;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::Fake;

#[async_std::test]
async fn test_insert() -> anyhow::Result<()> {
    let connection = get_postgres_connection().await?;
    let mut conn = connection.conn().await?;

    let new_user = NewUser {
        username: &Username(EN).fake::<String>(),
        first_name: Some(FirstName(EN).fake()),
        last_name: Some(LastName(EN).fake()),
        email: &FreeEmail(EN).fake::<String>(),
        encrypted_password: &Password(EN, 8..20).fake::<String>(),
        phone: None,
        status: 1,
    };

    let dt_per_insert = Utc::now().naive_utc();

    let (id, created_at) = insert(&mut conn, &new_user).await?;

    assert!(id > 0);
    assert!(created_at >= dt_per_insert);
    assert!(created_at <= Utc::now().naive_utc());

    Ok(())
}
