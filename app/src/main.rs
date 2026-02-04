use crate::{infrastructure::file_repository::FileRepository, ui::axum::ServeExt};
use anyhow::Result;
use libsql::params;
use logic::file_service::FileService;
use tokio::fs;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod infrastructure;
mod logic;
mod ui;

#[derive(Clone)]
pub struct Context {
    file_service: FileService,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let data_root = "./data".to_string();
    fs::create_dir_all(&data_root).await?;

    let files_root = format!("{}/files", data_root);
    fs::create_dir_all(&files_root).await?;

    let database_path = format!("{}/drive.db", data_root);
    let database = libsql::Builder::new_local(&database_path).build().await?;

    let db = database.connect()?;
    db.query("SELECT 1", params![]).await?;

    let file_repository = FileRepository {};
    let file_service = FileService {
        file_repository,
        db,
        files_root,
    };

    let context = Context { file_service };

    context.serve().await?;

    Ok(())
}
