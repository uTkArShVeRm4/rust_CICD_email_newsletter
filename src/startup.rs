use axum::{
    response::Result,
    routing::Router,
    routing::{get, post},
    serve::Serve,
};
use sqlx::PgConnection;

use crate::{routes::*, state::AppState};
use tokio::net::TcpListener;

pub async fn run(
    listener: TcpListener,
    connection: PgConnection,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let state = AppState::new(connection);

    // build our application with a single route
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state);

    let server = axum::serve(listener, app);

    Ok(server)
}
