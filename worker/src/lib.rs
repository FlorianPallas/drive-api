use std::time::Duration;

use anyhow::Result;
use deadpool_libsql::{Connection, Pool};
use libsql::params;
use queue::{Event, Queue};
use tokio::{fs, io::AsyncReadExt};
use tracing::{debug, info, warn};

pub async fn run(pool: Pool) -> Result<()> {
    info!("Worker started");

    let files_root = "./data/files".to_string();

    let queue = Queue {};

    loop {
        let db = pool.get().await?;

        let Some(job) = queue.dequeue(&db).await? else {
            tokio::time::sleep(Duration::from_secs(1)).await;
            continue;
        };

        let job_id = job.id;

        let handle_result: Result<(), anyhow::Error> = match job.payload {
            Event::FileUploaded { file_id } => analyze_file(&files_root, &db, file_id).await,
        };

        match handle_result {
            Ok(()) => {
                debug!("Job {} completed successfully", job_id);
                queue.complete(&db, job_id).await?;
            }
            Err(err) => {
                warn!("Job {} failed: {}", job_id, &err);
                queue.fail(&db, job_id).await?;
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn analyze_file(files_root: &str, db: &Connection, file_id: i64) -> Result<()> {
    let file_path = format!("{}/{}", files_root, file_id);
    let file = fs::File::open(&file_path).await?;

    let metadata = file.metadata().await?;
    let size = metadata.len();

    let mut buffer = Vec::with_capacity(8096);
    file.take(8096).read_to_end(&mut buffer).await?;

    let mime_type = infer::get(&buffer).map(|t| t.mime_type());
    debug!("Inferred MIME type: {:?} for file {}", mime_type, file_id);

    db.execute(
        "UPDATE files SET mime_type = ?, size = ? WHERE id = ?",
        params![mime_type, size, file_id],
    )
    .await?;

    Ok(())
}
