use anyhow::Result;
use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use tracing::error;

use crate::{Context, logic::file_service::File};

pub fn route(context: Context) -> Router {
    Router::new()
        .route("/files", get(list_files))
        .route("/files", post(upload_file))
        .route("/files/{id}", get(download_file))
        .route("/files/{id}", delete(delete_file))
        .route("/files/trash", get(list_trashed_files))
        .route("/files/trash", post(trash_file))
        .route("/files/trash/{id}", delete(restore_file))
        .with_state(context)
}

async fn list_files(State(context): State<Context>) -> Result<Json<Vec<File>>, ApiError> {
    let files = context.file_service.list_files(false).await?;

    Ok(Json(files))
}

async fn list_trashed_files(State(context): State<Context>) -> Result<Json<Vec<File>>, ApiError> {
    let files = context.file_service.list_files(true).await?;

    Ok(Json(files))
}

async fn upload_file(
    State(context): State<Context>,
    mut multipart: Multipart,
) -> Result<(), ApiError> {
    while let Some(field) = multipart.next_field().await? {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await?;

        context.file_service.upload_file(&file_name, &data).await?;
    }

    Ok(())
}

async fn download_file(
    State(context): State<Context>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let (file, data) = context.file_service.download_file(id).await?;

    let response = axum::response::Response::builder()
        .header("Content-Type", "image/png")
        .header(
            "Content-Disposition",
            format!("inline; filename=\"{}\"", file.name),
        )
        .body(Body::from(data))?;

    Ok(response)
}

async fn delete_file(State(context): State<Context>, Path(id): Path<i64>) -> Result<(), ApiError> {
    context.file_service.delete_file(id).await?;
    Ok(())
}

async fn trash_file(State(context): State<Context>, Path(id): Path<i64>) -> Result<(), ApiError> {
    context.file_service.trash_file(id).await?;
    Ok(())
}

async fn restore_file(State(context): State<Context>, Path(id): Path<i64>) -> Result<(), ApiError> {
    context.file_service.restore_file(id).await?;
    Ok(())
}

struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!("ApiError: {}", self.0);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
