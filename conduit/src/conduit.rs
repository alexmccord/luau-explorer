use std::io::Write;
use std::process::{Child, Command, Stdio};

pub enum ConduitRequest {
    VM { code: String },
    Lint { code: String },
    Check { code: String },
    // Note that the response of json encoding is not formatted.
    // If you want to pretty it up, do it yourself.
    Json { code: String },
    // The result is not quite perfect. I have an idea of how to
    // improve it, but let's live with it for now.
    Hydrate { code: String },
    // TODO: bytecode dump
    // TODO: autocomplete?
    // TODO: what else?
}

impl ConduitRequest {
    // We probably should get some crate that generates the discriminant values for us.
    // Also we need to be starting from 1, otherwise on Windows, a null byte terminates the stdin.
    fn discriminant(&self) -> u8 {
        match self {
            ConduitRequest::VM { .. } => 1,
            ConduitRequest::Lint { .. } => 2,
            ConduitRequest::Check { .. } => 3,
            ConduitRequest::Json { .. } => 4,
            ConduitRequest::Hydrate { .. } => 5,
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.into()
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
                // 10 bytes for right padding is because i32's upper bound in string form is that long.
                bytestr.extend(format!("{:0>10}", code.len() as i32).as_bytes());
                bytestr.extend(code.as_bytes());
            }
            ConduitRequest::Lint { code } => {
                bytestr.extend(format!("{:0>10}", code.len() as i32).as_bytes());
                bytestr.extend(code.as_bytes());
            }
            ConduitRequest::Check { code } => {
                bytestr.extend(format!("{:0>10}", code.len() as i32).as_bytes());
                bytestr.extend(code.as_bytes());
            }
            ConduitRequest::Json { code } => {
                bytestr.extend(format!("{:0>10}", code.len() as i32).as_bytes());
                bytestr.extend(code.as_bytes());
            }
            ConduitRequest::Hydrate { code } => {
                bytestr.extend(format!("{:0>10}", code.len() as i32).as_bytes());
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
        let process = Command::new(Conduit::which())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        Conduit { process }
    }

    pub fn dispatch(request: ConduitRequest) -> Result<Output, String> {
        let conduit = Conduit::new();

        let mut stdin = match conduit.process.stdin.as_ref() {
            Some(stdin) => stdin,
            None => return Err("Backend process has no stdin handle?".into()),
        };

        if let Err(e) = stdin.write_all(request.into_bytes().as_slice()) {
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
