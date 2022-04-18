fn main() {
    let mut config = cmake::Config::new("../backend");
    config.build_target("backend");
    let dest = config.build();

    let backend_build_dir = dest.join("build").join(config.get_profile());
    let backend_exe_name = if backend_build_dir.join("backend.exe").exists() {
        "backend.exe"
    } else if backend_build_dir.join("backend").exists() {
        "backend"
    } else {
        unreachable!();
    };

    // Needless to say, I am not a fan of this.
    let deps_dir = dest.parent().unwrap().parent().unwrap().parent().unwrap().join("deps");
    if let Err(e) = std::fs::copy(backend_build_dir.join(backend_exe_name), deps_dir.join(backend_exe_name)) {
        panic!("Failed to copy: {}", e);
    }
}
