#[derive(thiserror::Error, Debug)]
pub enum UserStatusError {
    #[error("from error")]
    FromError {
        #[from]
        source: num_enum::TryFromPrimitiveError<super::UserStatus>,
    },
}
