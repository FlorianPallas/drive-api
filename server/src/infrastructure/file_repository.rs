use anyhow::Result;
use chrono::{DateTime, Utc};
use deadpool_libsql::Connection;
use libsql::{de::from_row, params};
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

        Ok(from_row::<FileEntityId>(&row)?.id)
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

    pub async fn list_files(&self, db: &Connection) -> Result<Vec<FileEntity>> {
        let mut rows = db.query("SELECT * FROM files", params![]).await?;

        let mut files = Vec::new();
        while let Some(row) = rows.next().await? {
            files.push(from_row(&row)?);
        }

        Ok(files)
    }

    pub async fn set_trashed_at(
        &self,
        db: &Connection,
        file_id: i64,
        trashed_at: DateTime<Utc>,
    ) -> Result<()> {
        db.execute(
            "UPDATE files SET trashed_at = ? WHERE id = ?",
            params![trashed_at, file_id],
        )
        .await?;

        Ok(())
    }

    pub async fn delete_file(&self, db: &Connection, file_id: i64) -> Result<()> {
        db.execute("DELETE FROM files WHERE id = ?", params![file_id])
            .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct FileEntityId {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct FileEntity {
    pub id: i64,
    pub path: String,
    pub size: u64,
    pub mime_type: String,
    pub trashed_at: Option<DateTime<Utc>>,
}
