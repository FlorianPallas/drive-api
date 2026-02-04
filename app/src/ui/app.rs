use anyhow::Result;
use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tracing::info;

pub fn route() -> Router {
    Router::new().route("/", get(index))
}

struct RenderError(anyhow::Error);

impl<E> From<E> for RenderError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        RenderError(err.into())
    }
}

impl IntoResponse for RenderError {
    fn into_response(self) -> Response {
        info!("RenderError: {}", self.0);

        let html = ErrorTemplate {
            status_code: 500,
            message: "Internal Server Error",
        }
        .render()
        .unwrap_or(
            StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .unwrap()
                .to_owned(),
        );
        (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
    }
}

async fn index() -> Result<impl IntoResponse, RenderError> {
    let html = IndexTemplate { name: "World" }.render()?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    status_code: u16,
    message: &'a str,
}
