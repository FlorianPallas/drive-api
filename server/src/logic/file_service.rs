use anyhow::Result;
use chrono::{DateTime, Utc};
use deadpool_libsql::Pool;
use queue::{Event, Queue};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::infrastructure::file_repository::FileRepository;

#[derive(Clone)]
pub struct FileService {
    pub file_repository: FileRepository,
    pub queue: Queue,
    pub files_root: String,
    pub pool: Pool,
}

impl FileService {
    pub async fn upload_file(&self, file_path: &str, content: &[u8]) -> Result<()> {
        let conn = self.pool.get().await?;

        let file_id = self.file_repository.insert_file(&conn, file_path).await?;

        let disk_path = format!("{}/{}", self.files_root, file_id);
        fs::write(disk_path, content).await?;

        self.queue
            .enqueue(&conn, &Event::FileUploaded { file_id })
            .await?;

        Ok(())
    }

    pub async fn head_file(&self, file_id: i64) -> Result<File> {
        let file_entity = self
            .file_repository
            .get_file(&self.pool.get().await?, file_id)
            .await?;

        let file = File {
            id: file_entity.id,
            name: file_entity
                .path
                .split("/")
                .last()
                .unwrap_or_default()
                .to_string(),
            size: file_entity.size,
            mime_type: file_entity.mime_type,
        };

        Ok(file)
    }

    pub async fn download_file(&self, file_id: i64) -> Result<(File, Vec<u8>)> {
        let file = self.head_file(file_id).await?;

        let disk_path = format!("{}/{}", self.files_root, file.id);
        let content = fs::read(disk_path).await?;

        Ok((file, content))
    }

    pub async fn list_files(&self, trash: bool) -> Result<Vec<File>> {
        let file_entities = self
            .file_repository
            .list_files(&self.pool.get().await?)
            .await?;

        let files = file_entities
            .into_iter()
            .filter(|file| {
                if trash {
                    file.trashed_at.is_some()
                } else {
                    file.trashed_at.is_none()
                }
            })
            .map(|file| File {
                id: file.id,
                name: file.path.split("/").last().unwrap_or_default().to_string(),
                size: file.size,
                mime_type: file.mime_type,
            })
            .collect();

        Ok(files)
    }

    pub async fn trash_file(&self, file_id: i64) -> Result<()> {
        self.file_repository
            .set_trashed_at(&self.pool.get().await?, file_id, chrono::Utc::now())
            .await?;

        Ok(())
    }

    pub async fn delete_file(&self, file_id: i64) -> Result<()> {
        let file = self.head_file(file_id).await?;

        let disk_path = format!("{}/{}", self.files_root, file.id);
        fs::remove_file(disk_path).await?;

        self.file_repository
            .delete_file(&self.pool.get().await?, file_id)
            .await?;

        Ok(())
    }

    pub async fn restore_file(&self, file_id: i64) -> Result<()> {
        self.file_repository
            .delete_file(&self.pool.get().await?, file_id)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub mime_type: String,
    pub size: u64,
}
