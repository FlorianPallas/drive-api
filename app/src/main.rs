use crate::{infrastructure::file_repository::FileRepository, ui::axum::ServeExt};
use libsql::params;
use logic::file_service::FileService;

mod infrastructure;
mod logic;
mod ui;

#[derive(Clone)]
pub struct Context {
    file_service: FileService,
}

#[tokio::main]
async fn main() {
    let database = libsql::Builder::new_local("./data/drive.db")
        .build()
        .await
        .unwrap();

    let db = database.connect().unwrap();
    db.query("SELECT 1", params![]).await.unwrap();

    let file_repository = FileRepository {};
    let file_service = FileService {
        file_repository,
        db,
        data_path: "./data/files".to_string(),
    };

    let context = Context { file_service };

    context.serve().await;
}
