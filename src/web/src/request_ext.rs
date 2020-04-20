use crate::{decode_token, State};
use tide::{http_types, http_types::StatusCode, Request};

//

pub trait RequestAuthenticationExt {
    fn require_authentication(&self) -> Result<i64, http_types::Error>;
}

#[derive(thiserror::Error, Debug)]
enum RequestAuthenticationExtError {
    #[error("token missing")]
    TokenMissing,
    #[error("token invalid")]
    TokenInvalid(#[from] jsonwebtoken::errors::Error),
}

impl RequestAuthenticationExt for Request<State> {
    fn require_authentication(&self) -> Result<i64, http_types::Error> {
        let values = self.header(&"Authorization".parse()?).ok_or_else(|| {
            http_types::Error::new(
                StatusCode::Unauthorized,
                RequestAuthenticationExtError::TokenMissing,
            )
        })?;

        let value = values.first().ok_or_else(|| {
            http_types::Error::new(
                StatusCode::Unauthorized,
                RequestAuthenticationExtError::TokenMissing,
            )
        })?;

        let token_claims = decode_token(value.as_str(), &self.state().jwt_hs_secret)?;

        let user_id = token_claims.user_id;

        Ok(user_id)
    }
}
