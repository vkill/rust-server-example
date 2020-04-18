use super::{CreateUserError, GetUserByEmailAndPasswordError, User, UserForCreate};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: UserForCreate) -> Result<User, CreateUserError>;

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, GetUserByEmailAndPasswordError>;
}
