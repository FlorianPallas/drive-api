use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Job {
    AnalyzeFile { file_id: i64 },
}

use anyhow::Result;
use chrono::Local;
use clorinde::{
    deadpool_postgres::Client,
    queries::queue::{self, JobEntity},
    types::JobStatus,
};

#[derive(Clone)]
pub struct JobRepository {}

impl JobRepository {
    pub async fn enqueue(&self, client: &Client, payload: &serde_json::Value) -> Result<()> {
        let now = Local::now().naive_local();
        queue::enqueue().bind(client, &payload, &now, &now).await?;
        Ok(())
    }

    pub async fn dequeue(&self, client: &Client) -> Result<Option<JobEntity>> {
        let now = Local::now().naive_local();
        let event = queue::dequeue().bind(client, &now).opt().await?;
        Ok(event)
    }

    pub async fn update_status(&self, client: &Client, id: i64, status: &JobStatus) -> Result<()> {
        let now = Local::now().naive_local();
        queue::update_status()
            .bind(client, status, &now, &id)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, client: &Client, id: i64) -> Result<()> {
        queue::delete().bind(client, &id).await?;
        Ok(())
    }
}
