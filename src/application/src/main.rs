use application::Configuration;
use async_std::task;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let configuration =
        Configuration::new(PathBuf::default()).expect("Failed to load configuration");
    env_logger::init();

    let http_server_addr = format!(
        "{}:{}",
        configuration.http_server.host, configuration.http_server.port
    );
    task::block_on(run_http_server(http_server_addr))?;

    Ok(())
}

async fn run_http_server(addr: String) -> anyhow::Result<()> {
    let http_server = web::get_http_server();
    http_server.listen(addr).await?;

    Ok(())
}
