// script to create rpc files from grpc proto
//
// tries to autodetect the grpc-gateway/third_party/googleapis path
// if this fails prints a way to check-out this locally, but builds
// with the current rpc files.

use protoc_rust_grpc;
use std::path::{Path, PathBuf};

fn main() {
    // the rpc.protos need files from this
    let gateway_name = "grpc-gateway";
    let apis = "grpc-gateway/third_party/googleapis";
    let git_url = "https://github.com/grpc-ecosystem/grpc-gateway.git";

    // construct an include path
    let work = "work";
    let usr_inc = "/usr/include";
    let local_inc = "/usr/local/include";
    let lnd = "protos";

    let includes = [work, usr_inc, local_inc, lnd];

    // search for gateway apis
    let api_root = includes.iter().find(|d| Path::new(d).join(apis).is_dir());

    // local file paths
    let source = "protos/rpc.proto";
    let dest_dir = "protos";

    // run code generation or print warning
    match api_root {
        Some(d) => {
            let api_path = Path::new(d).join(apis);

            println!("cargo:rerun-if-changed={}", source);

            // generate rpc files
            protoc_rust_grpc::Codegen::new()
                .out_dir(dest_dir)
                .includes(&[
                    api_path,
                    PathBuf::from(usr_inc),
                    PathBuf::from(local_inc),
                    PathBuf::from(lnd),
                ])
                .input(source)
                .rust_protobuf(true)
                .run()
                .expect("protoc-rust-grpc")
        }
        None => {
            println!(
                "cargo:warning=using existing rpc files, as missing: {}",
                apis
            );
            println!(
                "cargo:warning=suggest: mkdir {0:}; git clone {1:} {0:}/{2:}",
                work, git_url, gateway_name
            );
        }
    }
}
