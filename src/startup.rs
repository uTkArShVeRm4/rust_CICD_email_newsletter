use axum::{
    response::Result,
    routing::Router,
    routing::{get, post},
    serve::Serve,
};

use crate::routes::*;
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<Serve<Router, Router>, std::io::Error> {
    // build our application with a single route
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let server = axum::serve(listener, app);

    Ok(server)
}
