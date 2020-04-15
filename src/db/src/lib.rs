mod connection;
pub use connection::Connection;

pub mod users;

pub type Result<T> = std::result::Result<T, sqlx::Error>;
