use anyhow::Result;
use chrono::NaiveDateTime;
use clorinde::{
    deadpool_postgres::Client,
    queries::files::{self, FileEntity},
};

#[derive(Clone)]
pub struct FileRepository {}

impl FileRepository {
    pub async fn insert_file(&self, client: &Client, file_path: &str) -> Result<i64> {
        let id = files::insert_file().bind(client, &file_path).one().await?;
        Ok(id)
    }

    pub async fn get_file(&self, client: &Client, file_id: i64) -> Result<FileEntity> {
        let file = files::get_file().bind(client, &file_id).one().await?;
        Ok(file)
    }

    pub async fn list_files(&self, client: &Client) -> Result<Vec<FileEntity>> {
        let files = files::list_files().bind(client).all().await?;
        Ok(files)
    }

    pub async fn set_trashed_at(
        &self,
        client: &Client,
        file_id: i64,
        trashed_at: Option<NaiveDateTime>,
    ) -> Result<()> {
        files::set_trashed_at()
            .bind(client, &trashed_at, &file_id)
            .await?;
        Ok(())
    }

    pub async fn delete_file(&self, client: &Client, file_id: i64) -> Result<()> {
        files::delete_file().bind(client, &file_id).await?;
        Ok(())
    }
}
