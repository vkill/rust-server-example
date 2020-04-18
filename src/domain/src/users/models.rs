use crate::UserPassword;
use crate::UserStatus;

pub struct UserForCreate {
    pub username: String,
    pub email: String,
    pub password: UserPassword,
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
