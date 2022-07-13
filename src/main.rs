use axum::response::IntoResponse;
use axum::routing::get_service;
use std::io;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

async fn handle_error(err: io::Error) -> impl IntoResponse {
    format!("error: {}", err)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), axum::BoxError> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 13892));
    axum::Server::bind(&addr)
        .serve(
            get_service(ServeDir::new("."))
                .handle_error(handle_error)
                .into_make_service(),
        )
        .await?;
    Ok(())
}
