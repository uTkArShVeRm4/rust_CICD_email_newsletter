use crate::state::AppState;
use axum::extract::State;
use axum::response::Result;
use axum::Form;
use serde::Deserialize;
use sqlx::query;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(app_state): State<AppState>,
    Form(form): Form<FormData>,
) -> Result<()> {
    let connection_arc = app_state.connection();
    let mut connection = connection_arc.lock().await;

    match query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(&mut *connection)
    .await
    {
        Ok(_res) => Ok(()),
        Err(_) => Err("Insert subscriber query failed".into()),
    }
}
