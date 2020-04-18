use crate::{to_domain_database_error, W};
use async_trait::async_trait;
use futures_util::future::TryFutureExt;

#[async_trait]
impl domain::UserRepository for crate::Repository {
    async fn create_user(
        &self,
        user: domain::UserForCreate,
    ) -> Result<domain::User, domain::CreateUserError> {
        let status = domain::UserStatus::Active;

        let new_user = db::NewUser {
            username: &user.username,
            first_name: None,
            last_name: None,
            email: &user.email,
            encrypted_password: &user.password.hash(),
            phone: None,
            status: status.clone() as i32,
        };

        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let (user_id, _) = db::users::insert(&mut conn, &new_user)
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let W(user) = (new_user, user_id, status).into();

        Ok(user)
    }
}

impl From<(db::NewUser<'_>, domain::UserID, domain::UserStatus)> for W<domain::User> {
    fn from(t: (db::NewUser<'_>, domain::UserID, domain::UserStatus)) -> Self {
        let (new_user, user_id, status) = t;

        let user = domain::User {
            id: user_id,
            username: new_user.username.into(),
            email: new_user.email.into(),
            status: status,
            profile: domain::UserProfile {
                first_name: new_user.first_name.map(|x| x.into()),
                last_name: new_user.last_name.map(|x| x.into()),
                phone: new_user.phone.map(|x| x.into()),
            },
        };

        W(user)
    }
}
