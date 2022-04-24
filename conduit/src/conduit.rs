use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};

pub enum ConduitRequest {
    VM { code: String },
}

impl ConduitRequest {
    // We probably should get some crate that generates the discriminant values for us.
    // Also we need to be starting from 1, otherwise on Windows, a null byte terminates the stdin.
    fn discriminant(&self) -> u8 {
        match self {
            ConduitRequest::VM { .. } => 1,
        }
    }
}

impl Into<Vec<u8>> for ConduitRequest {
    fn into(self) -> Vec<u8> {
        // We don't particularly care about being memory efficient,
        // because we're not transporting this over the wire.
        let mut bytestr = Vec::new();
        bytestr.push(self.discriminant());

        match self {
            ConduitRequest::VM { code } => {
                bytestr.extend(format!("{:0>8}", code.len()).as_bytes());
                bytestr.extend(code.as_bytes());
            }
        }

        bytestr
    }
}

#[derive(PartialEq, Debug)]
pub struct Output {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub struct Conduit {
    process: Child,
    stdin: ChildStdin,
}

impl Conduit {
    #[cfg(target_os = "windows")]
    fn which() -> &'static str {
        "backend.exe"
    }

    #[cfg(not(target_os = "windows"))]
    fn which() -> std::path::PathBuf {
        let dir = std::env::current_exe().unwrap();
        let cwd = dir.parent().unwrap();
        cwd.join("backend")
    }

    fn new() -> Conduit {
        let mut process = Command::new(Conduit::which())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdin = process.stdin.take().unwrap();

        Conduit { process, stdin }
    }

    pub fn launch(request: ConduitRequest) -> Result<Output, String> {
        let mut conduit = Conduit::new();
        let buf: Vec<u8> = request.into();

        if let Err(e) = conduit.stdin.write_all(buf.as_slice()) {
            return Err(e.to_string());
        }

        match conduit.process.wait_with_output() {
            Ok(result) => Ok(Output {
                status: result.status.code(),
                stdout: normalize(
                    &mut String::from_utf8(result.stdout).unwrap_or("Got invalid UTF-8.".into()),
                ),
                stderr: normalize(
                    &mut String::from_utf8(result.stderr).unwrap_or("Got invalid UTF-8.".into()),
                ),
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn normalize(s: &mut String) -> String {
    s.retain(|c| c != '\r');
    s.to_owned()
}
