use rusty_email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    println!(
        "Listening on 127.0.0.1:{}",
        listener.local_addr().unwrap().port()
    );
    let server = run(listener).await?;
    let _ = server.await;
    Ok(())
}
