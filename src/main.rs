use rusty_email_newsletter::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to get configurations");

    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(address.clone()).await?;
    println!("Listening on {}", address);
    let server = run(listener, connection).await?;
    let _ = server.await;
    Ok(())
}
