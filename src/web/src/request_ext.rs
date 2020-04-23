use crate::{decode_token, State};
use tide::{http_types, Request};

//

pub trait RequestAuthenticationExt {
    fn require_authentication(&self) -> crate::Result<i64>;
}

#[derive(thiserror::Error, Debug)]
enum RequestAuthenticationExtError {
    #[error("token missing")]
    TokenMissing,
    #[error("token invalid")]
    TokenInvalid(#[from] jsonwebtoken::errors::Error),
}

impl RequestAuthenticationExt for Request<State> {
    fn require_authentication(&self) -> crate::Result<i64> {
        let token = self
            .header(&"Authorization".parse().unwrap())
            .map(|values| values.first().map(|value| value.as_str().to_string()))
            .unwrap_or(Some("".to_string()))
            .ok_or_else(|| {
                tide::Error::new(
                    http_types::StatusCode::Unauthorized,
                    RequestAuthenticationExtError::TokenMissing,
                )
            })?;

        let token_claims = decode_token(&token, &self.state().jwt_hs_secret)?;

        let user_id = token_claims.user_id;

        Ok(user_id)
    }
}
