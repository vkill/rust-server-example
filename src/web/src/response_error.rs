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
// common
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
