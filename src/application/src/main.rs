use application::Configuration;
use async_std::task;
use dotenv::dotenv;
use repository::{Connection as DBConnection, Postgres, Repository};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let proj_path = PathBuf::default();

    dotenv().ok();
    if proj_path.join("src/db/.env").exists() {
        dotenv::from_filename(proj_path.join("src/db/.env")).ok();
    }

    let configuration = Configuration::new(proj_path).expect("Failed to load configuration");
    env_logger::init();

    let http_server_addr = format!(
        "{}:{}",
        configuration.http_server.host, configuration.http_server.port
    );
    task::block_on(run_http_server(http_server_addr))?;

    Ok(())
}

async fn run_http_server(addr: String) -> anyhow::Result<()> {
    let postgres_connection =
        DBConnection::<Postgres>::new(&dotenv::var("DATABASE_URL").ok().expect("")).await?;
    let repository = Repository {
        postgres_connection,
    };

    let postgres_connection =
        DBConnection::<Postgres>::new(&dotenv::var("DATABASE_URL").ok().expect("")).await?;

    let repository_for_graphql = Repository {
        postgres_connection,
    };

    let jwt_hs_secret = dotenv::var("JWT_HS_SECRET").ok().expect("");

    let state = web::State::new(repository, repository_for_graphql, jwt_hs_secret);

    let http_server = web::get_http_server(state);
    http_server.listen(addr).await?;

    Ok(())
}
