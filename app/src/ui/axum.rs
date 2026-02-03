use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};

use crate::Context;

pub trait ServeExt {
    async fn serve(self);
}

impl ServeExt for Context {
    async fn serve(self) {
        let app = Router::new()
            .route("/", post(upload_file))
            .route("/{file_id}", get(download_file))
            .with_state(self);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        println!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    }
}

async fn upload_file(
    State(context): State<Context>,
    mut multipart: Multipart,
) -> Result<(), AppError> {
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
) -> Result<impl IntoResponse, AppError> {
    let (file, data) = context.file_service.download_file(file_id).await?;

    let response = axum::response::Response::builder()
        .header("Content-Type", "image/png")
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file.name),
        )
        .body(Body::from(data))?;

    Ok(response)
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
