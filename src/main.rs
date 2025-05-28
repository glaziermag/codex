mod grpc;
mod http;
mod task_manager;

use crate::grpc::service;
use axum::Server as AxumServer;
use task_manager::TaskManager;
use tonic::transport::Server;
use tracing::info;

use opentelemetry::runtime::Tokio;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry_otlp::WithExportConfig;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::new_exporter().tonic();
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .install_batch(Tokio)?;

    let otel_layer = OpenTelemetryLayer::new(tracer);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .try_init()?;
    Ok(())
}

use std::net::SocketAddr;

pub mod task {
    pub mod v1 {
        tonic::include_proto!("task.v1");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;
    let manager = TaskManager::new();

    let grpc_service = service(manager.clone());
    let http_router = http::router(manager.clone());

    let grpc_addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let http_addr: SocketAddr = "0.0.0.0:3000".parse()?;

    info!("starting gRPC server on {}", grpc_addr);
    info!("starting HTTP server on {}", http_addr);

    let grpc_server = Server::builder().add_service(grpc_service).serve(grpc_addr);
    let http_server = AxumServer::bind(&http_addr).serve(http_router.into_make_service());

    tokio::try_join!(grpc_server, http_server)?;
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
