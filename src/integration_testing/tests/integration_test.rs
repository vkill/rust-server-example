use assert_cmd::prelude::*;
use async_std::task;
use std::panic;
use std::process::Command;
use std::time::Duration;

#[async_std::test]
async fn test_all() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("application").expect("failed to get cargo bin");
    let mut child = cmd
        .env("RUST_BACKTRACE", "1")
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

    let string = surf::get("http://127.0.0.1:8080/hello")
        .recv_string()
        .await
        .ok();
    assert_eq!(string, Some("Hello, world!".into()));

    child.kill().expect("failed to kill process");

    Ok(())
}
