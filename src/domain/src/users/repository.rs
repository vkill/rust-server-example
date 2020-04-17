use crate::{CreateUserError, User, UserForCreate};

pub trait UserRepository {
    fn create_user(&self, user: UserForCreate) -> Result<User, CreateUserError>;
}
