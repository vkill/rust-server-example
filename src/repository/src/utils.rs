pub fn to_domain_repository_error<E>(e: db::Error) -> domain::RepositoryError<E>
where
    E: domain::RepositoryLogicError,
{
    domain::RepositoryError::<E>::DBError(anyhow::Error::from(e))
}

pub struct W<T>(pub T);
