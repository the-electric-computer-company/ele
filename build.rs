extern crate protoc;
extern crate protoc_rust_grpc;

use protoc::Protoc;

fn main() {
  if let Err(err) = Protoc::from_env_path().check() {
    eprintln!("`protoc` not in $PATH, skipping code generation: {}", err);
  } else {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
      out_dir: "src",
      includes: &["proto"],
      input: &["proto/node.proto"],
      rust_protobuf: true, // also generate protobuf messages, not just services
      ..Default::default()
    }).expect("protoc-rust-grpc");
  }
}
