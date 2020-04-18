#[derive(thiserror::Error, Debug)]
pub enum CreateUserError {
    #[error("Something went wrong.")]
    DatabaseError(#[from] crate::DatabaseError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserByEmailAndPasswordError {
    #[error("There is no user with the email and password you specified")]
    NotFound,
    #[error("Failed to process password")]
    PasswordError(#[from] crate::UserPasswordError),
    #[error("Something went wrong.")]
    DatabaseError(#[from] crate::DatabaseError),
}
