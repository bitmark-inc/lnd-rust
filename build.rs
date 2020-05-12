extern crate protoc_rust_grpc;

fn main() {
    let apis = "work/grpc-gateway/third_party/googleapis";
    let usr_inc = "/usr/include";
    let local_inc = "/usr/local/include";
    let lnd = "protos";
    let source = "protos/rpc.proto";
    let dest_dir = "protos";

    println!("cargo:rerun-if-changed={}", source);

    // builder style
    protoc_rust_grpc::Codegen::new()
        .out_dir(dest_dir)
        .includes(&[apis, usr_inc, local_inc, lnd])
        .input(source)
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc")
}
