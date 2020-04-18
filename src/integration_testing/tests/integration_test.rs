use assert_cmd::prelude::*;
use async_std::task;
use std::panic;
use std::process::Command;
use std::time::Duration;

use fake::faker::internet::raw::*;
use fake::locales::EN;
use fake::Fake;

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
    let resp_body_string = surf::get(format!("{}/hello", http_server_base_url))
        .recv_string()
        .await
        .ok();
    assert_eq!(resp_body_string, Some("Hello, world!".into()));

    //
    let resp_body_string = surf::get(format!("{}/server_ip", http_server_base_url))
        .recv_string()
        .await
        .ok();
    assert_eq!(resp_body_string.expect("").contains(r#""ip""#), true);

    //
    let req_body_json = serde_json::json!({
        "user": {
            "username": &Username(EN).fake::<String>(),
            "email": &FreeEmail(EN).fake::<String>(),
            "password": &Password(EN, 8..20).fake::<String>()
        }
    });
    let mut res = surf::post(format!("{}/users", http_server_base_url))
        .body_json(&req_body_json)?
        .await
        .ok()
        .expect("");
    assert_eq!(res.status(), 201);
    let resp_body_string = res.body_string().await.ok();
    assert_eq!(resp_body_string.expect("").contains(r#""token""#), true);

    //
    child.kill().expect("failed to kill process");

    Ok(())
}
