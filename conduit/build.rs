use std::path::PathBuf;

struct Location {
    build_dir: PathBuf,
    name: &'static str,
}

#[cfg(target_os = "windows")]
fn locate_executable(dest: PathBuf, profile: &str) -> Location {
    let backend_build_dir = dest.join("build").join(profile);
    Location {
        build_dir: backend_build_dir,
        name: "backend.exe",
    }
}

#[cfg(not(target_os = "windows"))]
fn locate_executable(dest: PathBuf, _profile: &str) -> Location {
    let backend_build_dir = dest.join("build");
    Location {
        build_dir: backend_build_dir,
        name: "backend",
    }
}

fn main() {
    let mut config = cmake::Config::new("../backend");
    config.build_target("backend");

    if cfg!(windows) {
        // cmake-rs for no good reason whatsoever erases /EHsc but Luau depends on it.
        config.cxxflag("/EHsc");
    } else if cfg!(unix) {
        // gcc 7.1 to 9.4 has a problem with unused local warnings
        // will be removed once Luau release includes this in their CMakeLists.txt.
        config.cxxflag("-Wno-unused");
    }

    let dest = config.build();
    let Location { build_dir, name } = locate_executable(dest.clone(), config.get_profile());

    if !build_dir.join(name).exists() {
        panic!("No {} found in {}?", name, build_dir.display());
    }

    let backend_binary_path = build_dir.join(name);

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
