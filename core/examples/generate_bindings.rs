//! Generate Kotlin and Swift bindings for mepassa-core
//!
//! Usage: cargo run --example generate_bindings

use camino::Utf8PathBuf;
use std::path::PathBuf;
use std::process;

fn main() {
    // Ensure library is built first
    println!("Building library...");
    let build_status = process::Command::new("cargo")
        .args(&["build", "--lib", "--release"])
        .status()
        .expect("Failed to execute cargo build");

    if !build_status.success() {
        eprintln!("Failed to build library");
        process::exit(1);
    }

    // Setup paths
    let crate_root = std::env::current_dir().expect("Failed to get current dir");
    let udl_path = crate_root.join("src/mepassa.udl");
    let out_dir = crate_root.join("../target/bindings");

    // Find the compiled library
    let lib_path = crate_root.join("../target/release/libmepassa_core.dylib");

    if !lib_path.exists() {
        eprintln!("Library not found at: {}", lib_path.display());
        process::exit(1);
    }

    // Create output directory
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    println!("\nGenerating Swift bindings...");
    println!("UDL file: {}", udl_path.display());
    println!("Library: {}", lib_path.display());
    println!("Output: {}", out_dir.display());

    // Generate Swift bindings using library mode
    let result = uniffi_bindgen::library_mode::generate_bindings(
        &lib_path,
        None, // crate_name - will be detected from library
        &uniffi_bindgen::BindingsConfig::default(),
        None, // config_file_override
        &out_dir,
        false, // try_format_code
    );

    match result {
        Ok(_) => {
            println!("✓ Swift bindings generated successfully!");
            println!("\nOutput directory: {}", out_dir.display());
            println!("\nGenerated files:");
            // List generated files
            if let Ok(entries) = std::fs::read_dir(&out_dir) {
                for entry in entries.flatten() {
                    println!("  - {}", entry.path().display());
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to generate bindings: {}", e);
            process::exit(1);
        }
    }
}
