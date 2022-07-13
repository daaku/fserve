use axum::response::IntoResponse;
use axum::routing::get_service;
use clap::Parser;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;

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
    let args = Args::parse();
    axum::Server::bind(&args.addr)
        .serve(
            get_service(ServeDir::new(&args.dir))
                .handle_error(handle_error)
                .into_make_service(),
        )
        .await?;
    Ok(())
}
