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

        let db_new_user = db::NewUser {
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

        let (user_id, _) = db::users::insert(&mut conn, &db_new_user)
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let W(user) = (db_new_user, user_id, status).into();

        Ok(user)
    }

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<domain::User, domain::GetUserByEmailAndPasswordError> {
        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let db_user = db::users::find_by_email(&mut conn, email)
            .map_err(|e| match e {
                db::Error::RowNotFound => domain::GetUserByEmailAndPasswordError::NotFound,
                _ => to_domain_database_error(e).into(),
            })
            .await?;

        if !domain::UserPassword::from_hash(db_user.clone().encrypted_password).verify(password)? {
            return Err(domain::GetUserByEmailAndPasswordError::NotFound);
        }

        let W(user) = db_user.into();

        Ok(user)
    }

    async fn get_user_by_id(
        &self,
        id: domain::UserID,
    ) -> Result<domain::User, domain::GetUserByIDError> {
        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let db_user = db::users::find_by_id(&mut conn, id)
            .map_err(|e| match e {
                db::Error::RowNotFound => domain::GetUserByIDError::NotFound,
                _ => to_domain_database_error(e).into(),
            })
            .await?;

        let W(user) = db_user.into();

        Ok(user)
    }

    async fn update_user(
        &self,
        mut user: domain::User,
        user_profile: domain::UserProfile,
    ) -> Result<domain::User, domain::DatabaseError> {
        let db_update_user = db::UpdateUser {
            username: &user.username,
            first_name: user_profile.first_name.as_deref(),
            last_name: user_profile.last_name.as_deref(),
            phone: user_profile.phone.as_deref(),
        };

        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_database_error(e))
            .await?;

        let _ = db::users::update(&mut conn, user.id, &db_update_user)
            .map_err(|e| to_domain_database_error(e))
            .await?;

        user.profile.first_name = db_update_user.first_name.map(|x| x.to_owned());
        user.profile.last_name = db_update_user.last_name.map(|x| x.to_owned());
        user.profile.phone = db_update_user.phone.map(|x| x.to_owned());

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

impl From<db::User> for W<domain::User> {
    fn from(db_user: db::User) -> Self {
        let user = domain::User {
            id: db_user.id,
            username: db_user.username,
            email: db_user.email,
            status: domain::UserStatus::Active, // TODO
            profile: domain::UserProfile {
                first_name: db_user.first_name.map(|x| x.into()),
                last_name: db_user.last_name.map(|x| x.into()),
                phone: db_user.phone.map(|x| x.into()),
            },
        };

        W(user)
    }
}
