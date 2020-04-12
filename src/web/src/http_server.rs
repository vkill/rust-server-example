use crate::State;
use tide::Server;

pub fn get_app() -> Server<State> {
    let state = State {};
    let mut app = Server::with_state(state);
    add_routes(&mut app);
    app
}

fn add_routes(app: &mut Server<State>) {
    app.at("/hello").get(|_| async move { "Hello, world!" });
}
