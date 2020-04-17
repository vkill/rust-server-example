#[derive(thiserror::Error, Debug)]
pub enum CreateUserError {
    #[error("Something went wrong.")]
    DatabaseError(#[from] crate::DatabaseError),
}
