// Build script for mepassa-core
// Compiles Protocol Buffers definitions into Rust code

use std::io::Result;

fn main() -> Result<()> {
    // Compile Protocol Buffers
    let proto_files = ["../proto/messages.proto"];
    let proto_include = ["../proto"];

    prost_build::Config::new()
        .out_dir("src/protocol/generated")
        .compile_protos(&proto_files, &proto_include)?;

    // Tell Cargo to rerun if proto files change
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }

    Ok(())
}
