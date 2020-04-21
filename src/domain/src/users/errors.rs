use crate::RepositoryLogicError;

#[derive(thiserror::Error, Debug)]
pub enum CreateUserError {
    #[error("This email is exists")]
    EmailExists,
    #[error("Failed to process password")]
    PasswordError(#[from] crate::UserPasswordError),
}
impl RepositoryLogicError for CreateUserError {}

#[derive(thiserror::Error, Debug)]
pub enum GetUserByEmailAndPasswordError {
    #[error("There is no user with the email and password you specified")]
    NotFound,
    #[error("Failed to process password")]
    PasswordError(#[from] crate::UserPasswordError),
    #[error("Failed to process status")]
    StatusError(#[from] crate::UserStatusError),
}
impl RepositoryLogicError for GetUserByEmailAndPasswordError {}

#[derive(thiserror::Error, Debug)]
pub enum GetUserByIDError {
    #[error("There is no user with the email and password you specified")]
    NotFound,
    #[error("Failed to process status")]
    StatusError(#[from] crate::UserStatusError),
}
impl RepositoryLogicError for GetUserByIDError {}
