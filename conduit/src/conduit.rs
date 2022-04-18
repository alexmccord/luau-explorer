use std::process::{Command, Child, Stdio};

pub struct Conduit {
    pub(crate) process: Child,
}

impl Conduit {
    #[cfg(target_os = "windows")]
    fn launch() -> Command {
        Command::new("backend.exe")
    }

    #[cfg(not(target_os = "windows"))]
    fn launch() -> Command {
        let dir = std::env::current_exe().unwrap();
        let cwd = dir.parent().unwrap();
        Command::new(cwd.join("backend"))
    }

    pub fn new() -> Conduit {
        let child = Conduit::launch()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        Conduit { process: child }
    }
}
