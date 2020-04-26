use crate::State;
use tide::Server;

pub fn get_app(state: State) -> Server<State> {
    let mut app = Server::with_state(state);
    add_routes(&mut app);
    app
}

fn add_routes(app: &mut Server<State>) {
    app.at("/").get(|_| async move { Ok("It Works!") });
    app.at("/hello").get(|_| async move { Ok("Hello, world!") });

    app.at("/server_ip")
        .get(|req| async move { crate::others::server_ip(req).await });

    app.at("/users")
        .post(|req| async move { crate::users::sign_up(req).await });

    app.at("/users/sign_in")
        .post(|req| async move { crate::users::sign_in(req).await });

    app.at("/users/me")
        .patch(|req| async move { crate::users::update_profile(req).await });

    app.at("/graphql")
        .get(|req| async move { crate::graphql::graphql_playground(req).await })
        .post(|req| async move { crate::graphql::graphql_post(req).await })
        .at("/graphiql")
        .get(|req| async move { crate::graphql::graphql_graphiql(req).await });
}
