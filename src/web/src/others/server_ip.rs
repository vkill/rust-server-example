use crate::{ResponseError, State};
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

pub async fn server_ip(_: Request<State>) -> Result<Response, ResponseError> {
    let body_json: HTTPBinIPResponseBody = surf::get("https://httpbin.org/ip").recv_json().await?;

    let resp = Response::new(200).body_json(&body_json)?;

    Ok(resp)
}

#[derive(Deserialize, Serialize)]
struct HTTPBinIPResponseBody {
    #[serde(rename(deserialize = "origin"))]
    ip: String,
}
