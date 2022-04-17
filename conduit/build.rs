use cmake;

fn main() {
    cmake::Config::new("../backend")
        .build_target("backend")
        .build();
}
