use crate::UserPassword;

#[derive(Debug)]
pub struct UserForCreate {
    pub username: String,
    pub email: String,
    pub password: UserPassword,
}

#[derive(Debug)]
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug)]
pub enum UserStatus {
    Active = 1,
    Inactive = 2,
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub user_status: i32,
    pub profile: UserProfile,
}
