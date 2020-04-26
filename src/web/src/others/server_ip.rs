use crate::State;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, StatusCode};

pub async fn server_ip(_: Request<State>) -> tide::Result<Response> {
    let body_json: HTTPBinIPResponseBody = surf::get("https://httpbin.org/ip").recv_json().await?;

    let resp = Response::new(StatusCode::Ok).body_json(&body_json)?;

    Ok(resp)
}

#[derive(Deserialize, Serialize)]
struct HTTPBinIPResponseBody {
    #[serde(rename(deserialize = "origin"))]
    ip: String,
}
