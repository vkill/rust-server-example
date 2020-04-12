use async_std::task;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    task::block_on(run_web_http_server())?;

    Ok(())
}

async fn run_web_http_server() -> anyhow::Result<()> {
    let server = web::get_http_server();
    server.listen("127.0.0.1:8080").await?;

    Ok(())
}
