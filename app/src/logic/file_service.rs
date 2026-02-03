use anyhow::Result;
use libsql::Connection;
use tokio::fs;

use crate::infrastructure::file_repository::FileRepository;

#[derive(Clone)]
pub struct FileService {
    pub file_repository: FileRepository,
    pub db: Connection,
    pub data_path: String,
}

impl FileService {
    pub async fn upload_file(&self, file_path: &str, content: &[u8]) -> Result<()> {
        let file_id = self
            .file_repository
            .insert_file(&self.db, file_path)
            .await?;

        let file_path = format!("{}/{}", self.data_path, file_id);
        fs::write(file_path, content).await?;

        Ok(())
    }
}
