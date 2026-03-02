use std::path::{Component, Path, PathBuf};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::fs;
use tracing::{debug, warn};

use crate::infrastructure::file_repository::{FileEntity, FileRepository};

#[derive(Clone)]
pub struct FileService {
    pub file_repository: FileRepository,
    pub files_root: PathBuf,
    pub pool: PgPool,
}

impl FileService {
    pub async fn upload_file(&self, path: &str, content: &[u8]) -> Result<File> {
        let mut conn = self.pool.acquire().await?;

        let mime_type = infer::get(&content[..8096])
            .map(|t| t.mime_type())
            .unwrap_or("application/octet-stream");
        debug!("Inferred MIME type: {:?} for file {}", mime_type, path);

        let entity = self
            .file_repository
            .insert_file(&mut conn, path, content.len() as i64, mime_type)
            .await?;

        let disk_path = self
            .disk_path_for(&entity.path)
            .ok_or(anyhow!("invalid path"))?;

        if let Some(parent) = disk_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(disk_path, content).await?;

        Ok(entity.into())
    }

    pub async fn get_file(&self, id: i32) -> Result<Option<File>> {
        let mut conn = self.pool.acquire().await?;

        let entity = self.file_repository.get_file_by_id(&mut conn, id).await?;
        Ok(entity.map(File::from))
    }

    pub async fn list_files(&self, path: Option<&str>) -> Result<Vec<File>> {
        let mut conn = self.pool.acquire().await?;

        let entities = self
            .file_repository
            .list_files(&mut conn, path.unwrap_or_default())
            .await?;
        let files = entities.into_iter().map(File::from).collect();

        Ok(files)
    }

    pub async fn delete_file(&self, file_id: i32) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        let Some(entity) = self
            .file_repository
            .get_file_by_id(&mut conn, file_id)
            .await?
        else {
            return Err(anyhow::anyhow!("File not found"));
        };

        let disk_path = self
            .disk_path_for(&entity.path)
            .ok_or(anyhow!("invalid path"))?;
        if let Err(e) = fs::remove_file(disk_path).await {
            warn!("Failed to delete file on disk: {:?}", e)
        };

        self.file_repository
            .delete_file_by_id(&mut conn, file_id)
            .await?;

        Ok(())
    }

    fn disk_path_for(&self, path: &str) -> Option<PathBuf> {
        let path = Path::new(path);
        for component in path.components() {
            match component {
                Component::Normal(_) => continue,
                _ => return None,
            }
        }
        Some(self.files_root.join(path))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<FileEntity> for File {
    fn from(value: FileEntity) -> Self {
        Self {
            id: value.id,
            path: value.path,
            mime_type: value.mime_type,
            size: value.size,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
