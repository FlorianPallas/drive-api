use anyhow::Result;
use libsql::{Connection, de::from_row, params};
use serde::Deserialize;

#[derive(Clone)]
pub struct FileRepository {}

impl FileRepository {
    pub async fn insert_file(&self, db: &Connection, file_path: &str) -> Result<i64> {
        let mut rows = db
            .query(
                "INSERT INTO files (path) VALUES (?) RETURNING id",
                params![file_path],
            )
            .await?;

        let Some(row) = rows.next().await? else {
            return Err(anyhow::anyhow!("File not found"));
        };

        Ok(from_row(&row)?)
    }

    pub async fn get_file(&self, db: &Connection, file_id: i64) -> Result<FileEntity> {
        let mut rows = db
            .query("SELECT * FROM files WHERE id = ?", params![file_id])
            .await?;

        let Some(row) = rows.next().await? else {
            return Err(anyhow::anyhow!("File not found"));
        };

        Ok(from_row(&row)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct FileEntity {
    pub id: i64,
    pub path: String,
}
