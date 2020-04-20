use repository::domain;
use tide::{http_types, http_types::StatusCode, IntoResponse, Response};

//
#[derive(Debug)]
pub struct ResponseError {
    resp: Response,
}

impl ResponseError {
    pub fn new(resp: Response) -> Self {
        Self { resp }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        self.resp
    }
}

//
impl From<tide::Response> for ResponseError {
    fn from(resp: tide::Response) -> Self {
        Self { resp }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ResponseError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        let resp = Response::new(StatusCode::InternalServerError).body_string(e.to_string());
        Self { resp }
    }
}

impl From<http_types::Error> for ResponseError {
    fn from(e: http_types::Error) -> Self {
        unimplemented!()
    }
}

impl From<serde_json::error::Error> for ResponseError {
    fn from(e: serde_json::error::Error) -> Self {
        let resp = Response::new(StatusCode::InternalServerError).body_string(e.to_string());
        Self { resp }
    }
}

impl From<jsonwebtoken::errors::Error> for ResponseError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        let resp = Response::new(StatusCode::InternalServerError).body_string(e.to_string());
        Self::new(resp)
    }
}

impl From<validator::ValidationErrors> for ResponseError {
    fn from(e: validator::ValidationErrors) -> Self {
        let resp = Response::new(StatusCode::BadRequest).body_string(e.to_string());
        Self::new(resp)
    }
}

//
impl From<domain::UserPasswordError> for ResponseError {
    fn from(e: domain::UserPasswordError) -> Self {
        let resp = Response::new(StatusCode::InternalServerError).body_string(e.to_string());
        Self::new(resp)
    }
}

//
impl<E> From<domain::RepositoryError<E>> for ResponseError
where
    E: domain::RepositoryLogicError,
{
    fn from(e: domain::RepositoryError<E>) -> Self {
        match e {
            domain::RepositoryError::DBError(e) => {
                let resp =
                    Response::new(StatusCode::InternalServerError).body_string(e.to_string());
                Self::new(resp)
            }
            domain::RepositoryError::LogicError(e) => unimplemented!(),
        }
    }
}
