use super::*;
use crate::RepositoryNoneLogicError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(
        &self,
        user: UserForCreate,
    ) -> crate::RepositoryResult<User, RepositoryNoneLogicError>;

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> crate::RepositoryResult<User, GetUserByEmailAndPasswordError>;

    async fn get_user_by_id(&self, id: UserID) -> crate::RepositoryResult<User, GetUserByIDError>;

    async fn update_user(
        &self,
        user: User,
        user_profile: UserProfile,
    ) -> crate::RepositoryResult<User, RepositoryNoneLogicError>;
}
