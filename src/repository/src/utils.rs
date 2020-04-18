pub fn to_domain_database_error(e: db::Error) -> domain::DatabaseError {
    domain::DatabaseError::from(anyhow::Error::from(e))
}

pub struct W<T>(pub T);
