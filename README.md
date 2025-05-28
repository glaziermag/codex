# codex

This repository contains a minimal gRPC API exposing a `TaskService`.
Tasks are stored in PostgreSQL via `PostgresStore` and can be streamed to
clients via server side streaming.

## Building proto files

Python gRPC stubs are generated from `taskd_api/protos/task.proto` using
`grpcio-tools`:

```bash
python -m grpc_tools.protoc -I taskd_api/protos \
    --python_out=taskd_api/protos \
    --grpc_python_out=taskd_api/protos \
    taskd_api/protos/task.proto
```

## Running the server

```bash
python -m taskd_api.service
```

The service listens on port `50051` by default and requires a running
PostgreSQL database configured via the `DATABASE_URL` environment
variable.

## Observability

Tracing is exported using the OTLP protocol. Prometheus metrics are exposed on `/metrics` from the HTTP server. An example Grafana dashboard and a Prometheus `ServiceMonitor` can be found in the `monitoring/` directory.

To enable metrics locally, run the application and scrape `localhost:3000/metrics`.

## Development

Use `just` to run common tasks. The `dev` recipe starts the application and `ci` runs the checks used in CI:

```bash
just dev
just ci
```

