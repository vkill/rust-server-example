use super::{
    CreateUserError, GetUserByEmailAndPasswordError, GetUserByIDError, User, UserForCreate, UserID,
    UserProfile,
};
use crate::DatabaseError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: UserForCreate) -> Result<User, CreateUserError>;

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, GetUserByEmailAndPasswordError>;

    async fn get_user_by_id(&self, id: UserID) -> Result<User, GetUserByIDError>;

    async fn update_user(
        &self,
        user: User,
        user_profile: UserProfile,
    ) -> Result<User, DatabaseError>;
}
