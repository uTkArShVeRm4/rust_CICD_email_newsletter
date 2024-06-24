use axum::{response::Result, routing::get, serve::Serve, Router};

async fn health_check() -> Result<()> {
    Ok(())
}

pub async fn run() -> Result<Serve<Router, Router>, std::io::Error> {
    // build our application with a single route
    let app = Router::new().route("/health_check", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    let server = axum::serve(listener, app);

    Ok(server)
}
