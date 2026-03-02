use anyhow::Result;
use sqlx::PgPool;
use tracing::info;

pub async fn run(_pool: PgPool) -> Result<()> {
    info!("Worker started");
    Ok(())
}
