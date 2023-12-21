
use std::env;

fn main() {
    tonic_build::configure().build_client(true)
        .out_dir("src")
        .compile(
        &["src/proto/gateway.proto"], &["src/proto"]).unwrap();
}
