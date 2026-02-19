use std::ops::DerefMut;

use anyhow::Result;
use clorinde::{
    deadpool_postgres::{Config, Runtime},
    tokio_postgres::NoTls,
};
use tokio::fs;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

refinery::embed_migrations!("../migrations");

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,{}_server=debug,{}_worker=debug,tower_http=debug,axum::rejection=trace,refinery=info",
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

    info!("Connecting to database");
    let mut config = Config::new();
    config.url = Some("postgresql://postgres:postgres@localhost:5432/drive".to_string());
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    info!("Running migrations");
    let mut client = pool.get().await?;
    migrations::runner()
        .run_async(client.deref_mut().deref_mut())
        .await?;

    info!("Starting worker");
    let worker_pool = pool.clone();
    let worker_handle = tokio::spawn(async move { drive_worker::run(worker_pool).await });

    info!("Starting server");
    drive_server::run(pool).await.unwrap();

    let _ = tokio::join!(worker_handle).0?;

    Ok(())
}
