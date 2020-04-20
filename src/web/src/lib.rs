#[macro_use]
extern crate validator_derive;

mod http_server;
pub use http_server::get_app as get_http_server;

mod state;
use state::State;

type Result<T> = std::result::Result<T, tide::http_types::Error>;

mod token;
use token::{decode_token, encode_token};

mod request_ext;
use request_ext::*;

mod others;
mod users;
