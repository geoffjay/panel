fn main() {
    // Only watch the `dist/` directory for recompiling, preventing unnecessary
    // changes when we change files in other project subdirectories.
    println!("cargo:rerun-if-changed=dist");

    tauri_build::build()
}
