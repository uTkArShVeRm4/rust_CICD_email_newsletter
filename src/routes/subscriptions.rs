use axum::response::Result;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(_form_data): Form<FormData>) -> Result<()> {
    Ok(())
}
