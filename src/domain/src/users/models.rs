use crate::UserStatus;
use validator::Validate;

#[derive(Validate, Debug)]
pub struct CreateUserInput {
    #[validate(length(min = 4, max = 32))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

pub type UserID = i64;

pub struct User {
    pub id: UserID,
    pub username: String,
    pub email: String,
    pub status: UserStatus,
    pub profile: UserProfile,
}
