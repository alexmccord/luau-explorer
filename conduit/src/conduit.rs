use std::process::{Command, Child, Stdio};

pub struct Conduit {
    pub(crate) process: Child,
}

impl Conduit {
    pub fn new() -> Conduit {
        let mut command = Command::new("backend.exe");
        let child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        Conduit { process: child }
    }
}
