use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tracing::error;

use crate::Context;

pub fn route(context: Context) -> Router {
    Router::new()
        .route("/files", post(upload_file))
        .route("/files/{file_id}", get(download_file))
        .with_state(context)
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
    Path(file_id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let (file, data) = context.file_service.download_file(file_id).await?;

    let response = axum::response::Response::builder()
        .header("Content-Type", "image/png")
        .header(
            "Content-Disposition",
            format!("inline; filename=\"{}\"", file.name),
        )
        .body(Body::from(data))?;

    Ok(response)
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
