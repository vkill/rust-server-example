use crate::State;
use tide::Server;

pub struct HTTPServer {
    pub app: Server<State>,
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
impl HTTPServer {
    pub fn new() -> Self {
        let state = State {};
        let app = Server::with_state(state);
        Self { app }
    }

    pub fn configure(&mut self) {
        self.routes()
    }

    fn routes(&mut self) {
        self.app
            .at("/hello")
            .get(|_| async move { "Hello, world!" });
    }
}
