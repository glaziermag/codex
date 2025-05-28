fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/task/v1/task.proto"], &["proto"]).unwrap();
}
