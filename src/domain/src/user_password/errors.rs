#[derive(thiserror::Error, Debug)]
#[error("Failed to process password.")]
pub struct UserPasswordError {
    #[from]
    source: bcrypt::BcryptError,
}
