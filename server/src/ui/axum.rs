use tracing::info;

use crate::Context;

pub trait ServeExt {
    async fn serve(self) -> anyhow::Result<()>;
}

impl ServeExt for Context {
    async fn serve(self) -> anyhow::Result<()> {
        let api = crate::ui::api::route(self);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        info!("listening on http://{}", listener.local_addr()?);
        axum::serve(listener, api).await?;

        Ok(())
    }
}
