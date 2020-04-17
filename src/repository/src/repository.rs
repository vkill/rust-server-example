use crate::Connection;
use crate::Postgres;

pub struct Repository {
    pub postgres_connection: Connection<Postgres>,
}
