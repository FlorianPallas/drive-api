use std::time::Duration;

use anyhow::Result;
use clorinde::{
    deadpool_postgres::{Client, Pool},
    queries::files,
};
use queue::{Event, EventRepository};
use tokio::{fs, io::AsyncReadExt};
use tracing::{debug, info, warn};

pub async fn run(pool: Pool) -> Result<()> {
    info!("Worker started");

    let files_root = "./data/files".to_string();

    let event_repository = EventRepository {};

    loop {
        let db = pool.get().await?;

        let Some(job) = event_repository.dequeue(&db).await? else {
            tokio::time::sleep(Duration::from_secs(1)).await;
            continue;
        };

        let job_id = job.id;
        let payload = serde_json::from_str(&job.payload)?;

        let handle_result: Result<(), anyhow::Error> = match payload {
            Event::FileUploaded { file_id } => analyze_file(&files_root, &db, file_id).await,
        };

        match handle_result {
            Ok(()) => {
                debug!("Job {} completed successfully", job_id);
                event_repository.delete(&db, job_id).await?;
            }
            Err(err) => {
                warn!("Job {} failed: {}", job_id, &err);
                event_repository
                    .update_status(&db, job_id, "Failed")
                    .await?;
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn analyze_file(files_root: &str, client: &Client, file_id: i64) -> Result<()> {
    let file_path = format!("{}/{}", files_root, file_id);
    let file = fs::File::open(&file_path).await?;

    let metadata = file.metadata().await?;
    let size = metadata.len();

    let mut buffer = Vec::with_capacity(8096);
    file.take(8096).read_to_end(&mut buffer).await?;

    let mime_type = infer::get(&buffer)
        .map(|t| t.mime_type())
        .unwrap_or("application/octet-stream");
    debug!("Inferred MIME type: {:?} for file {}", mime_type, file_id);

    files::update_metadata()
        .bind(client, &mime_type, &(size as i64), &file_id)
        .await?;

    Ok(())
}
