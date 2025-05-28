mod task_manager;
mod grpc;
mod http;

use task_manager::TaskManager;
use axum::Server as AxumServer;
use tonic::transport::Server;
use crate::grpc::service;

use std::net::SocketAddr;

pub mod task {
    tonic::include_proto!("task");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = TaskManager::new();

    let grpc_service = service(manager.clone());
    let http_router = http::router(manager.clone());

    let grpc_addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let http_addr: SocketAddr = "0.0.0.0:3000".parse()?;

    let grpc_server = Server::builder().add_service(grpc_service).serve(grpc_addr);
    let http_server = AxumServer::bind(&http_addr).serve(http_router.into_make_service());

    tokio::try_join!(grpc_server, http_server)?;
    Ok(())
}
