//! Simple Swift bindings generator using uniffi
//!
//! Usage: cargo run --example simple_swift_gen

use std::path::PathBuf;
use std::process;

fn main() {
    println!("🔨 Generating Swift bindings for MePassa Core...");

    // Build the library first
    println!("📦 Building release library...");
    let build_result = process::Command::new("cargo")
        .args(&["build", "--lib", "--release"])
        .status()
        .expect("Failed to run cargo build");

    if !build_result.success() {
        eprintln!("❌ Failed to build library");
        process::exit(1);
    }

    // Setup paths
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let lib_path = workspace_root.join("target/release/libmepassa_core.dylib");
    let out_dir = workspace_root.join("ios/MePassa/Generated");

    // Verify library exists
    if !lib_path.exists() {
        eprintln!("❌ Library not found at: {}", lib_path.display());
        process::exit(1);
    }

    // Create output directory
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    println!("📚 Library: {}", lib_path.display());
    println!("📂 Output: {}", out_dir.display());

    // Call uniffi-bindgen as external command
    println!("\n🔧 Generating Swift bindings...");

    // Use the uniffi scaffolding to generate Swift code
    // We'll call the uniffi_bindgen library function directly
    uniffi::generate_component_scaffolding(
        "src/mepassa.udl",
        Some("mepassa"),
        Some("mepassaFFI"),
        &out_dir.to_string_lossy(),
        false,
    ).expect("Failed to generate scaffolding");

    println!("✅ Swift bindings generated successfully!");
    println!("\n📝 Generated files:");

    // List generated files
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                println!("   - {}", path.file_name().unwrap().to_string_lossy());
            }
        }
    }

    println!("\n🎯 Next steps:");
    println!("   1. Add generated files to Xcode project");
    println!("   2. Link libmepassa_core.a in Xcode");
    println!("   3. Import in Swift: import mepassa");
}
