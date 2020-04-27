use async_graphql::http::{graphiql_source, playground_source};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
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

pub async fn graphql_post(req: Request<State>) -> tide::Result<Response> {
    let schema = req.state().graphql_schema.0.clone();
    let user_id = req.require_authentication().ok();

    async_graphql_tide::graphql(req, schema, |mut query_builder| {
        if let Some(user_id) = user_id {
            query_builder = query_builder.data(ContextUserID(user_id))
        }

        query_builder
    })
    .await
}

pub async fn graphql_playground(_: Request<State>) -> tide::Result<Response> {
    let resp = Response::new(StatusCode::Ok)
        .body_string(playground_source("/graphql", None))
        .set_header(
            tide::http::headers::CONTENT_TYPE,
            tide::http::mime::HTML.to_string(),
        );

    Ok(resp)
}

pub async fn graphql_graphiql(_: Request<State>) -> tide::Result<Response> {
    let resp = Response::new(StatusCode::Ok)
        .body_string(graphiql_source("/graphql", None))
        .set_header(
            tide::http::headers::CONTENT_TYPE,
            tide::http::mime::HTML.to_string(),
        );

    Ok(resp)
}
