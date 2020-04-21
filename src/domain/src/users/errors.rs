use crate::RepositoryLogicError;

#[derive(thiserror::Error, Debug)]
pub enum CreateUserError {
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
}
impl RepositoryLogicError for GetUserByEmailAndPasswordError {}

#[derive(thiserror::Error, Debug)]
pub enum GetUserByIDError {
    #[error("There is no user with the email and password you specified")]
    NotFound,
}
impl RepositoryLogicError for GetUserByIDError {}
