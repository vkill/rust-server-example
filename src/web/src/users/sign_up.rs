use super::UserResponseBody;
use crate::{encode_token, State};
use repository::{domain, domain::UserRepository};
use serde::Deserialize;
use tide::{Request, Response, StatusCode};
use validator::Validate;

pub async fn sign_up(mut req: Request<State>) -> tide::Result<Response> {
    let req_body: SignUpRequestBody = req
        .body_json()
        .await
        .map_err(|e| tide::Error::new(StatusCode::BadRequest, e))?;

    let user: domain::CreateUserInput = req_body.into();

    let _ = user
        .validate()
        .map_err(|e| tide::Error::new(StatusCode::BadRequest, e))?;

    let repository = &req.state().repository;

    let user = repository.create_user(user).await.map_err(|e| match e {
        domain::CreateUserError::EmailExists => tide::Error::new(StatusCode::BadRequest, e),
        _ => e.into(),
    })?;

    let token = encode_token(user.id, &req.state().jwt_hs_secret)?;

    let resp_body: UserResponseBody = (user, token).into();

    let resp = Response::new(StatusCode::Created).body_json(&resp_body)?;

    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct SignUpRequestBody {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}

impl From<SignUpRequestBody> for domain::CreateUserInput {
    fn from(body: SignUpRequestBody) -> Self {
        Self {
            username: body.user.username,
            password: body.user.password,
            email: body.user.email,
        }
    }
}
