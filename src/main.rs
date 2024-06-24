use rusty_email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = run().await?;
    let _ = server.await;
    Ok(())
}
