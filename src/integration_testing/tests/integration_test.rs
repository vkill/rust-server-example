use assert_cmd::prelude::*;
use async_std::task;
use std::panic;
use std::process::Command;
use std::time::Duration;

#[async_std::test]
async fn test_all() -> anyhow::Result<()> {
    let port = 5001;
    let http_server_base_url = format!("http://127.0.0.1:{}", port);

    let mut cmd = Command::cargo_bin("application").expect("failed to get cargo bin");
    let mut child = cmd
        .env("RUST_BACKTRACE", "1")
        .env("APP_HTTP_SERVER__PORT", format!("{}", port))
        .spawn()
        .expect("failed to start process");
    panic::set_hook(Box::new(|_| {
        let output = Command::new("pkill")
            .args(&["-f", "target/debug/application"])
            .output()
            .expect("failed to run killall");
        println!("pkill output {:?}", output);
    }));

    task::sleep(Duration::from_secs(2)).await;

    //
    let body_string = surf::get(format!("{}/hello", http_server_base_url))
        .recv_string()
        .await
        .ok();
    assert_eq!(body_string, Some("Hello, world!".into()));

    //
    let body_string = surf::get(format!("{}/myip", http_server_base_url))
        .recv_string()
        .await
        .ok();
    assert_eq!(body_string.expect("").contains(r#""ip""#), true);

    //
    child.kill().expect("failed to kill process");

    Ok(())
}
