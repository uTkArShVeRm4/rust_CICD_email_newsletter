use std::{future::IntoFuture, time::Duration};

use rusty_email_newsletter::run;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    tokio::time::sleep(Duration::from_secs(1)).await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

fn spawn_app() {
    tokio::spawn(async {
        let server = run().await.expect("Failed to create server");
        server.into_future().await;
    });
}
