use std::path::PathBuf;

struct Location {
    build_dir: PathBuf,
    name: &'static str,
}

#[cfg(target_os = "windows")]
fn locate_executable(dest: PathBuf, profile: &str) -> Location {
    let backend_build_dir = dest.join("build").join(profile);

    if !backend_build_dir.join("backend.exe").exists() {
        panic!("No backend.exe found in {}, did compilation fail?", backend_build_dir.display());
    }

    Location { build_dir: backend_build_dir, name: "backend.exe" }
}

// Not yet sure if this works on MacOS too. I don't have a MacOS laptop upon which to test on.
#[cfg(not(target_os = "windows"))]
fn locate_executable(dest: PathBuf, _profile: &str) -> Location {
    let backend_build_dir = dest.join("build");

    if !backend_build_dir.join("backend").exists() {
        panic!("No backend found in {}, did compilation fail?", backend_build_dir.display());
    }

    Location { build_dir: backend_build_dir, name: "backend" }
}

fn main() {
    let mut config = cmake::Config::new("../backend");
    config.build_target("backend");
    let dest = config.build();

    let deps_dir = dest.parent().unwrap().parent().unwrap().parent().unwrap().join("deps");
    let Location { build_dir, name } = locate_executable(dest, config.get_profile());
    let backend_binary_path = build_dir.join(name);

    if let Err(e) = std::fs::copy(backend_binary_path, deps_dir.join(name)) {
        panic!("Failed to copy: {}", e);
    }
}
