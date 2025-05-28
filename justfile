# Development and CI tasks managed by just

# Run the application in development mode
# Usage: `just dev`

dev:
    cargo run

# Run checks used in CI pipelines
# Usage: `just ci`
ci:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test
    buf lint
    helm lint charts/

