use crate::{RequestAuthenticationExt, State};
use repository::{domain, domain::UserRepository};
use serde::Deserialize;
use tide::{http_types, Request, Response};

pub async fn update_profile(mut req: Request<State>) -> crate::Result<Response> {
    let user_id = req.require_authentication()?;

    let req_body: UpdateProfileRequestBody = req.body_json().await.map_err(|e| {
        Response::new(http_types::StatusCode::BadRequest).body_string(e.to_string())
    })?;
    let user_profile: domain::UserProfile = req_body.into();

    let repository = &req.state().repository;

    let user = repository
        .get_user_by_id(user_id)
        .await
        .map_err(|e| match e {
            domain::GetUserByIDError::NotFound => {
                http_types::Error::new(http_types::StatusCode::Forbidden, e)
            }
            _ => e.into(),
        })?;

    let _ = repository.update_user(user, user_profile).await?;

    let resp = Response::new(http_types::StatusCode::NoContent);

    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct UpdateProfileRequestBody {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}

impl From<UpdateProfileRequestBody> for domain::UserProfile {
    fn from(body: UpdateProfileRequestBody) -> Self {
        Self {
            first_name: body.user.first_name,
            last_name: body.user.last_name,
            phone: body.user.phone,
        }
    }
}
