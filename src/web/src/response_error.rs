use repository::domain;
use tide::{IntoResponse, Response};

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
    fn from(_: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self { resp }
    }
}

impl From<serde_json::error::Error> for ResponseError {
    fn from(_: serde_json::error::Error) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self { resp }
    }
}

impl From<jsonwebtoken::errors::Error> for ResponseError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self::new(resp)
    }
}

//
// domain
//
impl From<domain::UserPasswordError> for ResponseError {
    fn from(_: domain::UserPasswordError) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self::new(resp)
    }
}

impl From<domain::CreateUserError> for ResponseError {
    fn from(_: domain::CreateUserError) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self::new(resp)
    }
}

impl From<domain::GetUserByEmailAndPasswordError> for ResponseError {
    fn from(_: domain::GetUserByEmailAndPasswordError) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self::new(resp)
    }
}
