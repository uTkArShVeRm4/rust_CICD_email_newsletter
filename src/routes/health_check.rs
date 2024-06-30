use axum::response::Result;
pub async fn health_check() -> Result<()> {
    Ok(())
}
