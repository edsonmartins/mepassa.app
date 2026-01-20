//! Generate Kotlin and Swift bindings for mepassa-core
//!
//! Usage: cargo run --example generate_bindings

use camino::Utf8PathBuf;
use std::process;

fn main() {
    // Ensure library is built first
    println!("Building library...");
    let build_status = process::Command::new("cargo")
        .args(&["build", "--lib"])
        .status()
        .expect("Failed to execute cargo build");

    if !build_status.success() {
        eprintln!("Failed to build library");
        process::exit(1);
    }

    // Setup paths
    let udl_path = Utf8PathBuf::from("src/mepassa.udl");
    let out_dir = Utf8PathBuf::from("target/bindings");

    // Create output directory
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // Generate bindings for both Kotlin and Swift
    println!("\nGenerating Kotlin and Swift bindings...");

    let options = uniffi_bindgen::bindings::GenerateOptions {
        languages: vec![
            uniffi_bindgen::bindings::TargetLanguage::Kotlin,
            uniffi_bindgen::bindings::TargetLanguage::Swift,
        ],
        source: udl_path,
        out_dir: out_dir.clone(),
        config_override: None,
        format: false,
        crate_filter: None,
        metadata_no_deps: false,
    };

    match uniffi_bindgen::bindings::generate(options) {
        Ok(_) => {
            println!("✓ Bindings generated successfully!");
            println!("\nOutput directory: {}", out_dir);
            println!("\nGenerated files:");
            println!("  - Kotlin: target/bindings/uniffi/mepassa/mepassa.kt");
            println!("  - Swift: target/bindings/mepassaFFI.swift");
            println!("  - Swift header: target/bindings/mepassaFFI.h");
        }
        Err(e) => {
            eprintln!("✗ Failed to generate bindings: {}", e);
            process::exit(1);
        }
    }
}
