use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use serde::Deserialize;
use tracing::error;

use crate::Context;

#[derive(Deserialize)]
struct ListFilesInput {
    path: Option<String>,
}

pub fn route(context: Context) -> Router {
    Router::new()
        .route("/files", post(upload_file))
        .route("/files", get(list_files))
        .route("/files/{id}", get(get_file))
        .route("/files/{id}", delete(delete_file))
        .with_state(context)
}

async fn upload_file(
    State(context): State<Context>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let mut path: Option<String> = None;
    let mut data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default();

        match name {
            "path" => {
                path = Some(field.text().await?);
            }
            "file" => {
                data = Some(field.bytes().await?.to_vec());
            }
            _ => {}
        }
    }

    let file = match (path, data) {
        (Some(path), Some(bytes)) => context.file_service.upload_file(&path, &bytes).await?,
        _ => return Ok((StatusCode::BAD_REQUEST).into_response()),
    };

    Ok((StatusCode::CREATED, Json(file)).into_response())
}

async fn list_files(
    State(context): State<Context>,
    Query(query): Query<ListFilesInput>,
) -> Result<impl IntoResponse, ApiError> {
    let files = context
        .file_service
        .list_files(query.path.as_deref())
        .await?;
    Ok(Json(files).into_response())
}

async fn get_file(
    State(context): State<Context>,
    Path(file_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let file = context.file_service.get_file(file_id).await?;
    Ok(Json(file).into_response())
}

// async fn get_file(
//     State(context): State<Context>,
//     Path(path): Path<String>,
// ) -> Result<impl IntoResponse, ApiError> {
//     if path.ends_with("/download") {
//         let (file, data) = context.file_service.download_file(&path).await?;

//         let response = axum::response::Response::builder()
//             .header("Content-Type", "image/png")
//             .header(
//                 "Content-Disposition",
//                 format!("inline; filename=\"{}\"", file.name),
//             )
//             .body(Body::from(data))?;

//         return Ok(response);
//     }

//     match context.file_service.get_file(&path).await? {
//         Some(file) => Ok(Json(file).into_response()),
//         None => {
//             let files = context.file_service.list_files(&path).await?;
//             Ok(Json(files).into_response())
//         }
//     }
// }

async fn delete_file(
    State(context): State<Context>,
    Path(file_id): Path<i32>,
) -> Result<(), ApiError> {
    context.file_service.delete_file(file_id).await?;
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
