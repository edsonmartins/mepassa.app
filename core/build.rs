// Build script for mepassa-core
// 1. Compiles Protocol Buffers definitions into Rust code
// 2. Generates UniFFI scaffolding from UDL

fn main() {
    // 1. Compile Protocol Buffers
    let proto_files = ["../proto/messages.proto"];
    let proto_include = ["../proto"];

    prost_build::Config::new()
        .out_dir("src/protocol/generated")
        .compile_protos(&proto_files, &proto_include)
        .expect("Failed to compile protobuf");

    // Tell Cargo to rerun if proto files change
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }

    // 2. Generate UniFFI scaffolding
    uniffi::generate_scaffolding("src/mepassa.udl").expect("Failed to generate UniFFI scaffolding");
}
