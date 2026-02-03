use axum::{
    Router,
    extract::{Multipart, State},
    routing::post,
};

use crate::Context;

pub trait ServeExt {
    async fn serve(self);
}

impl ServeExt for Context {
    async fn serve(self) {
        let app = Router::new().route("/", post(handler)).with_state(self);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        println!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    }
}

async fn handler(State(context): State<Context>, mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        context
            .file_service
            .upload_file(&file_name, &data)
            .await
            .unwrap();
    }
}
