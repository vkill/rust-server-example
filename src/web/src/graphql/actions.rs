use async_graphql::http::{graphiql_source, playground_source, GQLRequest, GQLResponse};
use async_graphql::{EmptyMutation, EmptySubscription, IntoQueryBuilder, Schema};
use repository::Repository;
use tide::{Request, Response, StatusCode};

use super::{ContextUserID, QueryRoot};
use crate::{RequestAuthenticationExt, State};

pub struct GraphqlSchema(Schema<QueryRoot, EmptyMutation, EmptySubscription>);

impl GraphqlSchema {
    pub fn new(repository: Repository) -> Self {
        let scheme = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(repository)
            .finish();

        Self(scheme)
    }
}

//

pub async fn graphql_post(mut req: Request<State>) -> tide::Result<Response> {
    let gql_request: GQLRequest = req
        .body_json()
        .await
        .map_err(|e| tide::Error::new(StatusCode::BadRequest, e))?;

    let query_builder = gql_request
        .into_query_builder()
        .await
        .map_err(|e| tide::Error::new(StatusCode::BadRequest, e))?;

    let schema = &req.state().graphql_schema.0;

    let user_id = req.require_authentication().ok();

    let query_response = query_builder
        .data(ContextUserID(user_id))
        .execute(&schema)
        .await;

    let gql_response = GQLResponse(query_response);

    let resp = Response::new(StatusCode::Ok).body_json(&gql_response)?;

    Ok(resp)
}

pub async fn graphql_playground(_: Request<State>) -> tide::Result<Response> {
    let resp = Response::new(StatusCode::Ok)
        .body_string(playground_source("/graphql", None))
        .set_header("content-type".parse().unwrap(), "text/html");

    Ok(resp)
}

pub async fn graphql_graphiql(_: Request<State>) -> tide::Result<Response> {
    let resp = Response::new(StatusCode::Ok)
        .body_string(graphiql_source("/graphql"))
        .set_header("content-type".parse().unwrap(), "text/html");

    Ok(resp)
}
