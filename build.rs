use cmake;

fn main() {
    cmake::Config::new(".")
        .build_target("backend")
        .build();
}
