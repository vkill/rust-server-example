use tide::{IntoResponse, Response};

//
#[derive(Debug)]
pub struct ResponseError {
    resp: Response,
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        self.resp
    }
}

//
impl From<Box<dyn std::error::Error + Send + Sync>> for ResponseError {
    fn from(_: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self { resp }
    }
}

//
impl From<serde_json::error::Error> for ResponseError {
    fn from(_: serde_json::error::Error) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self { resp }
    }
}
