use super::UserResponseBody;
use crate::{encode_token, ResponseError, State};
use repository::{domain, domain::UserRepository};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use tide::{http_types::StatusCode, Request, Response};
use validator::Validate;

pub async fn sign_up(mut req: Request<State>) -> Result<Response, ResponseError> {
    let req_body: SignUpRequestBody = req
        .body_json()
        .await
        .map_err(|e| Response::new(StatusCode::BadRequest).body_string(e.to_string()))?;

    let _ = req_body.user.validate()?;

    let user: domain::UserForCreate = req_body.try_into()?;

    let repository = &req.state().repository;

    let user = repository.create_user(user).await?;

    let token = encode_token(user.id, &req.state().jwt_hs_secret)?;

    let resp_body: UserResponseBody = (user, token).into();

    let resp = Response::new(StatusCode::Created).body_json(&resp_body)?;

    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct SignUpRequestBody {
    user: User,
}

#[derive(Deserialize, Validate, Debug)]
struct User {
    #[validate(length(min = 4, max = 32))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}

impl TryFrom<SignUpRequestBody> for domain::UserForCreate {
    type Error = domain::UserPasswordError;

    fn try_from(body: SignUpRequestBody) -> Result<Self, Self::Error> {
        let user = Self {
            username: body.user.username,
            password: domain::UserPassword::from_clear_text(body.user.password)?,
            email: body.user.email,
        };
        Ok(user)
    }
}
