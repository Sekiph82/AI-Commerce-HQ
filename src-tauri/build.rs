use std::path::Path;

fn main() {
    // tauri-build validates that every entry in "resources" exists on disk.
    // backend.exe is produced by PyInstaller inside build-windows.ps1; it
    // won't exist in a clean checkout or during `cargo check`.
    // Create an empty placeholder so validation passes.
    // The real PyInstaller binary overwrites this placeholder before
    // `npm run tauri build` is invoked by the build script.
    let binaries_dir = Path::new("binaries");
    let placeholder = binaries_dir.join("backend.exe");
    if !placeholder.exists() {
        std::fs::create_dir_all(binaries_dir)
            .expect("failed to create src-tauri/binaries/");
        std::fs::write(&placeholder, b"")
            .expect("failed to create backend.exe placeholder");
        println!(
            "cargo:warning=Created empty backend.exe placeholder. \
             Run scripts/build-windows.ps1 to compile the real binary \
             before building the release installer."
        );
    }

    tauri_build::build()
}
