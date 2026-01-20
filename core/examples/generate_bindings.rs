//! Generate Kotlin and Swift bindings for mepassa-core
//!
//! Usage: cargo run --example generate_bindings

use std::path::PathBuf;

fn main() {
    let udl_file = PathBuf::from("src/mepassa.udl");
    let out_dir = PathBuf::from("target/bindings");

    // Create output directory
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    println!("Generating Kotlin bindings...");
    uniffi::generate_bindings(
        &udl_file,
        None, // config_file_override
        vec!["kotlin".to_string()],
        Some(&out_dir),
        None, // library_file
        "mepassa_core",
        false, // try_format_code
    ).expect("Failed to generate Kotlin bindings");

    println!("Generating Swift bindings...");
    uniffi::generate_bindings(
        &udl_file,
        None, // config_file_override
        vec!["swift".to_string()],
        Some(&out_dir),
        None, // library_file
        "mepassa_core",
        false, // try_format_code
    ).expect("Failed to generate Swift bindings");

    println!("Bindings generated successfully in {}", out_dir.display());
    println!("  - Kotlin: target/bindings/mepassa.kt");
    println!("  - Swift: target/bindings/mepassa.swift");
}
