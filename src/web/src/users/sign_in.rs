use super::UserResponseBody;
use crate::{encode_token, State};
use repository::{domain, domain::UserRepository};
use serde::Deserialize;
use tide::{Request, Response, Status, StatusCode};
use validator::Validate;

pub async fn sign_in(mut req: Request<State>) -> tide::Result<Response> {
    let req_body: SignInRequestBody = req.body_json().await.status(StatusCode::BadRequest)?;

    let _ = req_body.user.validate().status(StatusCode::BadRequest)?;

    let repository = &req.state().repository;

    let user = repository
        .get_user_by_email_and_password(&req_body.user.email, &req_body.user.password)
        .await
        .map_err(|e| match e {
            domain::GetUserByEmailAndPasswordError::NotFound => {
                tide::Error::new(StatusCode::NotFound, e)
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
