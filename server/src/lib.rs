use std::path::Path;

use crate::{infrastructure::file_repository::FileRepository, ui::axum::ServeExt};
use anyhow::Result;
use logic::file_service::FileService;
use sqlx::PgPool;
use tokio::fs;

mod infrastructure;
mod logic;
mod ui;

#[derive(Clone)]
pub struct Context {
    file_service: FileService,
}

pub async fn run(pool: PgPool) -> Result<()> {
    let data_root = Path::new("./data").canonicalize()?;
    fs::create_dir_all(&data_root).await?;

    let files_root = data_root.join("files").canonicalize()?;
    fs::create_dir_all(&files_root).await?;

    let file_repository = FileRepository {};
    let file_service = FileService {
        file_repository,
        pool,
        files_root,
    };

    let context = Context { file_service };

    context.serve().await?;

    Ok(())
}
