use anyhow::Result;
use libsql::{Connection, params};

#[derive(Clone)]
pub struct FileRepository {}

impl FileRepository {
    pub async fn insert_file(&self, db: &Connection, file_path: &str) -> Result<i64> {
        db.execute("INSERT INTO files (path) VALUES (?)", params![file_path])
            .await?;

        Ok(0)
    }
}
