use std::env;

fn main() {
    // Ensure OUT_DIR is set
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| {
        let default_dir = "target/debug/build";
        std::fs::create_dir_all(default_dir).expect("Failed to create build directory");
        default_dir.to_string()
    });
    println!("cargo:rustc-env=OUT_DIR={}", out_dir);
    
    tauri_build::build()
}
