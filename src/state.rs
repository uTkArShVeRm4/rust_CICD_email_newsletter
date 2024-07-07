use sqlx::PgConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    connection: Arc<Mutex<PgConnection>>,
}

impl AppState {
    pub fn new(connection: PgConnection) -> Self {
        AppState {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn connection(&self) -> Arc<Mutex<PgConnection>> {
        self.connection.clone()
    }
}
