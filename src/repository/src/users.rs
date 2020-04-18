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

        let (id, _) = db::users::insert(&mut conn, &new_user)
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let W(user) = (new_user, id, status).into();

        Ok(user)
    }
}

impl From<(db::NewUser<'_>, domain::UserID, domain::UserStatus)> for W<domain::User> {
    fn from(t: (db::NewUser<'_>, domain::UserID, domain::UserStatus)) -> Self {
        let (user, id, status) = t;

        let u = domain::User {
            id: id,
            username: user.username.into(),
            email: user.email.into(),
            status: status,
            profile: domain::UserProfile {
                first_name: user.first_name.map(|x| x.into()),
                last_name: user.last_name.map(|x| x.into()),
                phone: user.phone.map(|x| x.into()),
            },
        };

        W(u)
    }
}
