use std::path::PathBuf;
use std::process::Command;

struct Location {
    build_dir: PathBuf,
    name: &'static str,
}

#[cfg(target_os = "windows")]
fn locate_executable(dest: &PathBuf, profile: &str) -> Location {
    Location {
        build_dir: dest.join(profile),
        name: "backend.exe",
    }
}

#[cfg(not(target_os = "windows"))]
fn locate_executable(dest: &PathBuf, _profile: &str) -> Location {
    Location {
        build_dir: dest.to_owned(),
        name: "backend",
    }
}

fn main() {
    // Normally, we'd use cmake-rs. However, that doesn't work because the library for no good reason sets
    // - CMAKE_C_FLAGS
    // - CMAKE_C_FLAGS_DEBUG
    // - CMAKE_CXX_FLAGS
    // - CMAKE_CXX_FLAGS_DEBUG
    // - CMAKE_ASM_FLAGS
    // - CMAKE_ASM_FLAGS_DEBUG
    // which are typically left out, but yet the library overwrites them with -nologo, -M(T|D) and -Brepro.
    // Attempting to add flags back in to CMAKE_C_FLAGS and CMAKE_CXX_FLAGS end up futile.
    // Luau requires /EHsc, and MSVC requires us to pick the correct STATIC_CRT library.

    // Since it seems that cmake-rs, frustratingly, keeps on getting in the way, we'll just avoid it.
    // To do that, we cheat by resorting to spawning cmake directly.
    let dest = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut configure_command = Command::new("cmake");
    configure_command
        .arg(format!("-B{}", dest.display()))
        .arg("-S../backend");

    if let Err(e) = configure_command.status() {
        panic!("build step failed:\n{}", e);
    }

    let mut build_command = Command::new("cmake");
    build_command
        .current_dir(&dest)
        .arg("--build")
        .arg(".")
        .arg("--target")
        .arg("backend")
        .arg("--config")
        .arg("Debug") // TODO: Release. MinSizeRel. RelWithDebInfo.
        .arg("--parallel")
        .arg("16"); // TODO: Figure out something about 16. It's arbitrary.

    if let Err(e) = build_command.status() {
        panic!("build step failed:\n{}", e);
    }

    let Location { build_dir, name } = locate_executable(&dest, "Debug"); // TODO: profiles.
    let backend_binary_path = build_dir.join(name);

    if !backend_binary_path.exists() {
        panic!("'{}' is not found in '{}'", name, build_dir.display());
    }

    // Yeah.
    let deps_dir = dest
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("deps");

    if let Err(e) = std::fs::copy(backend_binary_path, deps_dir.join(name)) {
        panic!("Failed to copy: {}", e);
    }
}
