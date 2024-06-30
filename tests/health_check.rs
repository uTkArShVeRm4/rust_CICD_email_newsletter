use rusty_email_newsletter::run;
use std::{future::IntoFuture, time::Duration};
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to spawn tcp listener");

    let addr = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());

    tokio::spawn(async {
        let server = run(listener).await.expect("Failed to create server");

        let _ = server.into_future().await;
    });
    addr
}
