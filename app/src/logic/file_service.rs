use anyhow::Result;
use libsql::Connection;
use tokio::fs;

use crate::infrastructure::file_repository::FileRepository;

#[derive(Clone)]
pub struct FileService {
    pub file_repository: FileRepository,
    pub db: Connection,
    pub files_root: String,
}

impl FileService {
    pub async fn upload_file(&self, file_path: &str, content: &[u8]) -> Result<()> {
        let file_id = self
            .file_repository
            .insert_file(&self.db, file_path)
            .await?;

        let disk_path = format!("{}/{}", self.files_root, file_id);
        fs::write(disk_path, content).await?;

        Ok(())
    }

    pub async fn head_file(&self, file_id: i64) -> Result<File> {
        let file_entity = self.file_repository.get_file(&self.db, file_id).await?;

        let file = File {
            id: file_entity.id,
            name: file_entity
                .path
                .split("/")
                .last()
                .unwrap_or_default()
                .to_string(),
            path: file_entity.path,
        };

        Ok(file)
    }

    pub async fn download_file(&self, file_id: i64) -> Result<(File, Vec<u8>)> {
        let file = self.head_file(file_id).await?;

        let disk_path = format!("{}/{}", self.files_root, file.id);
        let content = fs::read(disk_path).await?;

        Ok((file, content))
    }
}

pub struct File {
    pub id: i64,
    pub name: String,
    pub path: String,
}
