use async_std::task;
use web::HTTPServer as WebHTTPServer;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    run_web_http_server()?;

    Ok(())
}

fn run_web_http_server() -> anyhow::Result<()> {
    let mut web_http_server = WebHTTPServer::new();
    web_http_server.configure();

    task::block_on(async {
        web_http_server.app.listen("127.0.0.1:8080").await?;
        Ok(())
    })
}
