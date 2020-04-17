use crate::to_domain_database_error;
use async_trait::async_trait;
use futures_util::future::TryFutureExt;

#[async_trait]
impl domain::UserRepository for crate::Repository {
    async fn create_user(
        &self,
        user: domain::UserForCreate,
    ) -> Result<domain::UserID, domain::CreateUserError> {
        let new_user = db::NewUser {
            username: &user.username,
            first_name: None,
            last_name: None,
            email: &user.email,
            encrypted_password: user.password.hash(),
            phone: None,
            user_status: domain::UserStatus::Active as i32,
        };

        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let (id, _) = db::users::insert(&mut conn, &new_user)
            .map_err(|e| to_domain_database_error(e))
            .await?;

        Ok(id)
    }
}
