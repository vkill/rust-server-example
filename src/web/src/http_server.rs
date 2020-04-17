use crate::State;
use tide::{IntoResponse, Response, Server};

pub fn get_app() -> Server<State> {
    let state = State {};
    let mut app = Server::with_state(state);
    add_routes(&mut app);
    app
}

fn add_routes(app: &mut Server<State>) {
    app.at("/hello").get(|_| async move { "Hello, world!" });

    app.at("/server_ip")
        .get(|req| async move { result_to_response(crate::others::server_ip(req).await) });

    app.at("/users")
        .post(|req| async move { result_to_response(crate::users::sign_up(req).await) });
}

//
fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response {
    match r {
        Ok(ir) => ir.into_response(),
        Err(ir) => {
            let resp = ir.into_response();
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
