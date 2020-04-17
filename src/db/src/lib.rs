mod connection;
pub use connection::Connection;
pub use sqlx::Error;
pub use sqlx::Postgres;

pub mod users;
pub use users::mapper::*;

pub type Result<T> = std::result::Result<T, sqlx::Error>;
