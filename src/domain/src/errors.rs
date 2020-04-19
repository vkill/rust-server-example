pub trait RepositoryLogicError: std::fmt::Debug + std::error::Error {}
#[derive(thiserror::Error, Debug)]
pub enum RepositoryNoneLogicError {}
impl RepositoryLogicError for RepositoryNoneLogicError {}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError<E>
where
    E: RepositoryLogicError + 'static,
{
    #[error("Something went wrong.")]
    LogicError(#[from] E),
    #[error("Something went wrong.")]
    DBError(#[from] anyhow::Error),
}
