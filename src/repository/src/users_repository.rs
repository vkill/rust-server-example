use crate::{to_domain_repository_error, W};
use async_trait::async_trait;
use domain::*;
use futures_util::future::TryFutureExt;

#[async_trait]
impl UserRepository for crate::Repository {
    async fn create_user(
        &self,
        user: CreateUserInput,
    ) -> domain::RepositoryResult<User, CreateUserError> {
        let status = UserStatus::Active;

        let user_password =
            UserPassword::from_clear_text(user.password).map_err(|e| CreateUserError::from(e))?;

        let db_new_user = db::NewUser {
            username: &user.username,
            first_name: None,
            last_name: None,
            email: &user.email,
            encrypted_password: user_password.hash(),
            phone: None,
            status: status.to_owned().into(),
        };

        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_repository_error(e))
            .await?;

        if db::users::exists_with_email(&mut conn, &db_new_user.email)
            .map_err(|e| to_domain_repository_error(e))
            .await?
        {
            return Err(CreateUserError::EmailExists.into());
        }

        let (user_id, _) = db::users::insert(&mut conn, &db_new_user)
            .map_err(|e| {
                if let db::Error::Database(db_e) = &e {
                    if db_e.code() == Some("23505") {
                        CreateUserError::EmailExists.into()
                    } else {
                        to_domain_repository_error(e)
                    }
                } else {
                    to_domain_repository_error(e)
                }
            })
            .await?;

        let W(user) = (db_new_user, user_id, status).into();

        Ok(user)
    }

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> domain::RepositoryResult<User, GetUserByEmailAndPasswordError> {
        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_repository_error(e))
            .await?;

        let db_user = db::users::find_by_email(&mut conn, email)
            .map_err(|e| match e {
                db::Error::RowNotFound => GetUserByEmailAndPasswordError::NotFound.into(),
                _ => to_domain_repository_error(e),
            })
            .await?;

        if !UserPassword::from_hash(db_user.to_owned().encrypted_password)
            .verify(password)
            .map_err(|e| GetUserByEmailAndPasswordError::from(e))?
        {
            return Err(GetUserByEmailAndPasswordError::NotFound.into());
        }

        let W(user) = db_user.into();

        Ok(user)
    }

    async fn get_user_by_id(&self, id: UserID) -> domain::RepositoryResult<User, GetUserByIDError> {
        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_repository_error(e))
            .await?;

        let db_user = db::users::find_by_id(&mut conn, id)
            .map_err(|e| match e {
                db::Error::RowNotFound => GetUserByIDError::NotFound.into(),
                _ => to_domain_repository_error(e),
            })
            .await?;

        let W(user) = db_user.into();

        Ok(user)
    }

    async fn update_user(
        &self,
        mut user: User,
        user_profile: UserProfile,
    ) -> domain::RepositoryResult<User, RepositoryNoneLogicError> {
        let db_update_user = db::UpdateUser {
            username: &user.username,
            first_name: user_profile.first_name.as_deref(),
            last_name: user_profile.last_name.as_deref(),
            phone: user_profile.phone.as_deref(),
        };

        let mut conn = self
            .postgres_connection
            .conn()
            .map_err(|e| to_domain_repository_error(e))
            .await?;

        let _ = db::users::update(&mut conn, user.id, &db_update_user)
            .map_err(|e| to_domain_repository_error(e))
            .await?;

        user.profile.first_name = db_update_user.first_name.map(|x| x.to_owned());
        user.profile.last_name = db_update_user.last_name.map(|x| x.to_owned());
        user.profile.phone = db_update_user.phone.map(|x| x.to_owned());

        Ok(user)
    }
}

impl From<(db::NewUser<'_>, UserID, UserStatus)> for W<User> {
    fn from(t: (db::NewUser<'_>, UserID, UserStatus)) -> Self {
        let (new_user, user_id, status) = t;

        let user = User {
            id: user_id,
            username: new_user.username.into(),
            email: new_user.email.into(),
            status: status,
            profile: UserProfile {
                first_name: new_user.first_name.map(|x| x.into()),
                last_name: new_user.last_name.map(|x| x.into()),
                phone: new_user.phone.map(|x| x.into()),
            },
        };

        W(user)
    }
}

impl From<db::User> for W<User> {
    fn from(db_user: db::User) -> Self {
        let user = User {
            id: db_user.id,
            username: db_user.username,
            email: db_user.email,
            status: UserStatus::Active, // TODO
            profile: UserProfile {
                first_name: db_user.first_name.map(|x| x.into()),
                last_name: db_user.last_name.map(|x| x.into()),
                phone: db_user.phone.map(|x| x.into()),
            },
        };

        W(user)
    }
}
