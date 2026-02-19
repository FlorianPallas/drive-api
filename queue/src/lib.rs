use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    FileUploaded { file_id: i64 },
    FileDownloaded { file_id: i64 },
    FileTrashed { file_id: i64 },
    FileRestored { file_id: i64 },
    FileDeleted { file_id: i64 },
}

use anyhow::Result;
use clorinde::{
    deadpool_postgres::Client,
    queries::queue::{self, EventEntity},
};

#[derive(Clone)]
pub struct EventRepository {}

impl EventRepository {
    pub async fn enqueue(&self, client: &Client, payload: &String, event_type: &str) -> Result<()> {
        queue::enqueue().bind(client, payload, &event_type).await?;
        Ok(())
    }

    pub async fn dequeue(
        &self,
        client: &Client,
        event_types: &[&str],
    ) -> Result<Option<EventEntity>> {
        let event = queue::dequeue().bind(client, &event_types).opt().await?;
        Ok(event)
    }

    pub async fn update_status(&self, client: &Client, id: i64, status: &str) -> Result<()> {
        queue::update_status().bind(client, &status, &id).await?;
        Ok(())
    }

    pub async fn delete(&self, client: &Client, id: i64) -> Result<()> {
        queue::delete().bind(client, &id).await?;
        Ok(())
    }
}
