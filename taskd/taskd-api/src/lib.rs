use axum::{routing::get, Router};
use tonic::transport::Server;

use taskd_core::Repository;

/// Build a simple HTTP router.
pub fn router() -> Router {
    Router::new().route("/health", get(|| async { "ok" }))
}

/// Placeholder gRPC server function.
pub async fn start_grpc(addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    Server::builder().serve(addr).await?;
    Ok(())
}
