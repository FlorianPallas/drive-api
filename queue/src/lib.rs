use anyhow::{Context, Result};
use libsql::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    FileUploaded { file_id: i64 },
}

#[derive(Clone)]
pub struct Queue {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JobEntity {
    id: i64,
    status: JobStatus,
    payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JobStatus {
    Pending,
    Running,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: i64,
    pub status: JobStatus,
    pub payload: Event,
}

impl Queue {
    pub async fn enqueue(&self, db: &Connection, task: &Event) -> Result<()> {
        let payload = serde_json::to_string(task).context("failed to serialize payload")?;

        db.execute("INSERT INTO jobs (payload) VALUES (?)", params![payload])
            .await?;

        Ok(())
    }

    pub async fn dequeue(&self, db: &Connection) -> Result<Option<Job>> {
        let mut rows = db
            .query(
                "UPDATE jobs SET status = 'Running' WHERE id = (SELECT id FROM jobs WHERE status = 'Pending' LIMIT 1) RETURNING *",
                params![],
            )
            .await?;

        let Some(row) = rows.next().await? else {
            return Ok(None);
        };

        let task = libsql::de::from_row::<JobEntity>(&row)?;

        let payload =
            serde_json::from_str(&task.payload).context("failed to deserialize payload")?;

        let queued = Job {
            id: task.id,
            status: task.status,
            payload,
        };

        Ok(Some(queued))
    }

    pub async fn complete(&self, db: &Connection, job_id: i64) -> Result<()> {
        db.execute(
            "DELETE FROM jobs WHERE id = ? AND status = 'Running'",
            params![job_id],
        )
        .await?;
        Ok(())
    }

    pub async fn fail(&self, db: &Connection, job_id: i64) -> Result<()> {
        db.execute(
            "UPDATE jobs SET status = 'Failed' WHERE id = ? AND status = 'Running'",
            params![job_id],
        )
        .await?;
        Ok(())
    }
}
