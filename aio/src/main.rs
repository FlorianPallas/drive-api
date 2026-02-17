use anyhow::Result;
use deadpool_libsql::{Manager, Pool};
use tokio::fs;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SCHEMA: &str = include_str!("../../schema.sql");

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,{}_server=debug,{}_worker=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME"), env!("CARGO_CRATE_NAME"), env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Creating data directory");
    let data_root = "./data".to_string();
    fs::create_dir_all(&data_root).await?;

    info!("Creating files directory");
    let files_root = format!("{}/files", data_root);
    fs::create_dir_all(&files_root).await?;

    info!("Creating database");
    let database_path = format!("{}/drive.db", data_root);
    let database = libsql::Builder::new_local(&database_path).build().await?;

    let manager = Manager::from_libsql_database(database);
    let pool = Pool::builder(manager).build()?;

    info!("Creating database schema");
    let conn = pool.get().await?;
    conn.execute_transactional_batch(SCHEMA).await?;

    info!("Starting worker");
    let x = pool.clone();
    let worker_handle = tokio::spawn(async move { drive_worker::run(x).await });

    info!("Starting server");
    drive_server::run(pool).await.unwrap();

    let _ = tokio::join!(worker_handle).0?;

    Ok(())
}
