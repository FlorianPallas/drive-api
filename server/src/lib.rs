use crate::{infrastructure::file_repository::FileRepository, ui::axum::ServeExt};
use anyhow::Result;
use clorinde::deadpool_postgres::Pool;
use logic::file_service::FileService;
use queue::JobRepository;
use tokio::fs;

mod infrastructure;
mod logic;
mod ui;

#[derive(Clone)]
pub struct Context {
    file_service: FileService,
}

pub async fn run(pool: Pool) -> Result<()> {
    let data_root = "./data".to_string();
    fs::create_dir_all(&data_root).await?;

    let files_root = format!("{}/files", data_root);
    fs::create_dir_all(&files_root).await?;

    let event_repository = JobRepository {};
    let file_repository = FileRepository {};
    let file_service = FileService {
        file_repository,
        job_repository: event_repository,
        pool,
        files_root,
    };

    let context = Context { file_service };

    context.serve().await?;

    Ok(())
}
