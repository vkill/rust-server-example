use super::UserResponseBody;
use crate::{encode_token, State};
use repository::{domain, domain::UserRepository};
use serde::Deserialize;
use tide::{http_types, http_types::StatusCode, Request, Response};
use validator::Validate;

pub async fn sign_in(mut req: Request<State>) -> crate::Result<Response> {
    let req_body: SignInRequestBody = req
        .body_json()
        .await
        .map_err(|e| Response::new(StatusCode::BadRequest).body_string(e.to_string()))?;

    let _ = req_body.user.validate()?;

    let repository = &req.state().repository;

    let user = repository
        .get_user_by_email_and_password(&req_body.user.email, &req_body.user.password)
        .await
        .map_err(|e| match e {
            domain::GetUserByEmailAndPasswordError::NotFound => {
                http_types::Error::new(StatusCode::NotFound, e)
            }
            _ => e.into(),
        })?;

    let token = encode_token(user.id, &req.state().jwt_hs_secret)?;

    let resp_body: UserResponseBody = (user, token).into();

    let resp = Response::new(StatusCode::Ok).body_json(&resp_body)?;

    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct SignInRequestBody {
    user: User,
}

#[derive(Deserialize, Validate, Debug)]
struct User {
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    password: String,
}
