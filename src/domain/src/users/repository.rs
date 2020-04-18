use super::{CreateUserError, User, UserForCreate};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: UserForCreate) -> Result<User, CreateUserError>;
}
