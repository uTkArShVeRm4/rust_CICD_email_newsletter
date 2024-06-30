use axum::{response::Result, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;

async fn health_check() -> Result<()> {
    Ok(())
}

pub async fn run(listener: TcpListener) -> Result<Serve<Router, Router>, std::io::Error> {
    // build our application with a single route
    let app = Router::new().route("/health_check", get(health_check));
    let server = axum::serve(listener, app);

    Ok(server)
}
