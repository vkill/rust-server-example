use crate::{decode_token, ResponseError, State};
use tide::{http_types::StatusCode, Request, Response};

pub trait RequestExt {
    fn require_authentication(&self) -> Result<i64, ResponseError>;
}

impl RequestExt for Request<State> {
    fn require_authentication(&self) -> Result<i64, ResponseError> {
        let values = self
            .header(&"Authorization".parse()?)
            .ok_or_else(|| ResponseError::new(Response::new(StatusCode::Unauthorized)))?;

        let value = values
            .first()
            .ok_or_else(|| ResponseError::new(Response::new(StatusCode::Unauthorized)))?;

        let token_claims = decode_token(value.as_str(), &self.state().jwt_hs_secret)?;

        let user_id = token_claims.user_id;

        Ok(user_id)
    }
}
