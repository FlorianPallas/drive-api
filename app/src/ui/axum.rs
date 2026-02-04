use axum::Router;

use crate::Context;

pub trait ServeExt {
    async fn serve(self) -> anyhow::Result<()>;
}

impl ServeExt for Context {
    async fn serve(self) -> anyhow::Result<()> {
        let app = crate::ui::app::route();
        let api = crate::ui::api::route(self);
        let router = Router::new().merge(app).nest("/api/v1", api);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        println!("listening on http://{}", listener.local_addr()?);
        axum::serve(listener, router).await?;

        Ok(())
    }
}
