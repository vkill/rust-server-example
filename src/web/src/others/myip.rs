use crate::{ResponseError, State};
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

pub async fn myip(_: Request<State>) -> Result<Response, ResponseError> {
    let uri = "https://api.myip.com/";
    let body_json: APIMyipResponseBody = surf::get(uri).recv_json().await?;

    let resp = Response::new(200).body_json(&body_json)?;

    Ok(resp)
}

#[derive(Deserialize, Serialize)]
struct APIMyipResponseBody {
    ip: String,
    country: String,
    cc: String,
}
