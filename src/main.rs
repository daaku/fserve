use axum::response::IntoResponse;
use axum::routing::get_service;
use clap::Parser;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn handle_error(err: io::Error) -> impl IntoResponse {
    format!("error: {}", err)
}

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Directory to serve files from.
    #[clap(short, long, default_value = ".")]
    dir: PathBuf,
    /// Address to bind to.
    #[clap(short, long, default_value = "127.0.0.1:8080")]
    addr: SocketAddr,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), axum::BoxError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "fserve=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    tracing::debug!("listening on {}", &args.addr);
    axum::Server::bind(&args.addr)
        .serve(
            get_service(ServeDir::new(&args.dir))
                .handle_error(handle_error)
                .layer(tower_http::compression::CompressionLayer::new())
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .into_make_service(),
        )
        .await?;
    Ok(())
}
