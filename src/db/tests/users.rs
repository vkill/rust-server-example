mod helpers;
use helpers::get_test_connection;

use db::users::mapper::*;
use db::users::query::*;

use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::Fake;

#[async_std::test]
async fn test_insert() -> anyhow::Result<()> {
    let connection = get_test_connection().await?;

    let new_user = NewUser {
        username: &Username(EN).fake::<String>(),
        first_name: Some(FirstName(EN).fake()),
        last_name: Some(LastName(EN).fake()),
        email: &FreeEmail(EN).fake::<String>(),
        encrypted_password: &Password(EN, 8..20).fake::<String>(),
        phone: None,
        user_status: Some(1),
    };

    let user = insert(&connection, &new_user).await?;

    assert!(user.id > 0);
    assert_eq!(user.email, new_user.email);

    Ok(())
}
