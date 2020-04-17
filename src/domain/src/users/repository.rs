use super::{CreateUserError, UserForCreate, UserID};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: UserForCreate) -> Result<UserID, CreateUserError>;
}
