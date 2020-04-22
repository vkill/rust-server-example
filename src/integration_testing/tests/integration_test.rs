use assert_cmd::prelude::*;
use async_std::task;
use std::panic;
use std::process::Command;
use std::time::Duration;

use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
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

    let email = FreeEmail(EN).fake::<String>();
    let password = Password(EN, 8..20).fake::<String>();
    //
    let req_body_json = serde_json::json!({
        "user": {
            "username": &Username(EN).fake::<String>(),
            "email": &email,
            "password": &password
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
    let req_body_json = serde_json::json!({
        "user": {
            "email": &email,
            "password": &password
        }
    });
    let mut res = surf::post(format!("{}/users/sign_in", http_server_base_url))
        .body_json(&req_body_json)?
        .await
        .ok()
        .expect("");
    assert_eq!(res.status(), 200);
    let resp_body_string = res.body_string().await.ok();
    assert_eq!(
        resp_body_string
            .as_deref()
            .expect("")
            .contains(r#""token""#),
        true
    );

    let resp_body_json: serde_json::Value = serde_json::from_str(&resp_body_string.expect(""))?;
    let token = resp_body_json["user"]["token"].as_str().expect("");

    //
    let req_body_json = serde_json::json!({
        "user": {
            "first_name": FirstName(EN).fake::<String>(),
            "last_name": LastName(EN).fake::<String>(),
            "phone": PhoneNumber(EN).fake::<String>()
        }
    });
    let res = surf::patch(format!("{}/users/me", http_server_base_url))
        .set_header("Authorization", token)
        .body_json(&req_body_json)?
        .await
        .ok()
        .expect("");
    assert_eq!(res.status(), 204);

    // graphql
    let req_body_json = serde_json::json!({
        "query": "{currentUserId}",
        "variables": null
    });
    let mut res = surf::post(format!("{}/graphql", http_server_base_url))
        .body_json(&req_body_json)?
        .await
        .ok()
        .expect("");
    println!("{:?}", res);
    assert_eq!(res.status(), 200);
    let resp_body_string = res.body_string().await.ok();
    println!("{:?}", resp_body_string);
    let resp_body_json: serde_json::Value = serde_json::from_str(&resp_body_string.expect(""))?;
    let user_id = resp_body_json["data"]["currentUserId"].as_str();
    assert_eq!(user_id, None);

    // graphql
    let req_body_json = serde_json::json!({
        "query": "{currentUserId}",
        "variables": null
    });
    let mut res = surf::post(format!("{}/graphql", http_server_base_url))
        .set_header("Authorization", token)
        .body_json(&req_body_json)?
        .await
        .ok()
        .expect("");
    println!("{:?}", res);
    assert_eq!(res.status(), 200);
    let resp_body_string = res.body_string().await.ok();
    println!("{:?}", resp_body_string);
    let resp_body_json: serde_json::Value = serde_json::from_str(&resp_body_string.expect(""))?;
    let user_id = resp_body_json["data"]["currentUserId"].as_str();
    assert_ne!(user_id, None);

    //
    child.kill().expect("failed to kill process");

    Ok(())
}
