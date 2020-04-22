use crate::State;
use tide::{http_types, IntoResponse, Response, Server};

pub fn get_app(state: State) -> Server<State> {
    let mut app = Server::with_state(state);
    add_routes(&mut app);
    app
}

fn add_routes(app: &mut Server<State>) {
    app.at("/").get(|_| async move { "It Works!" });
    app.at("/hello").get(|_| async move { "Hello, world!" });

    app.at("/server_ip")
        .get(|req| async move { result_to_response(crate::others::server_ip(req).await) });

    app.at("/users")
        .post(|req| async move { result_to_response(crate::users::sign_up(req).await) });

    app.at("/users/sign_in")
        .post(|req| async move { result_to_response(crate::users::sign_in(req).await) });

    app.at("/users/me")
        .patch(|req| async move { result_to_response(crate::users::update_profile(req).await) });

    app.at("/graphql")
        .get(|req| async move { crate::graphql::graphql_playground(req).await })
        .post(|req| async move { result_to_response(crate::graphql::graphql_post(req).await) })
        .at("/graphiql")
        .get(|req| async move { crate::graphql::graphql_graphiql(req).await });
}

//
fn result_to_response<T: IntoResponse>(r: crate::Result<T>) -> Response {
    match r {
        Ok(ir) => ir.into_response(),
        Err(mut ht_e) => {
            if let Some(_) = ht_e.downcast_ref::<validator::ValidationErrors>() {
                ht_e.set_status(http_types::StatusCode::BadRequest);
            }

            let resp = ht_e.into_response();
            if resp.status().is_success() {
                panic!(
                    "Attempted to yield error response with success code {:?}",
                    resp.status()
                )
            }
            resp
        }
    }
}
