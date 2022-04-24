use conduit::{Conduit, ConduitRequest, Output};

#[test]
fn launch_vm() {
    let request = ConduitRequest::VM {
        code: "print(5)".into(),
    };

    let result = Conduit::launch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(0),
            stdout: "5\n".into(),
            stderr: "".into()
        }
    );
}

#[test]
fn ensure_that_newlines_doesnt_matter() {
    let request = ConduitRequest::VM {
        code: "print(5)\r\nprint(5)".into(),
    };

    let result = Conduit::launch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(0),
            stdout: "5\n5\n".into(),
            stderr: "".into()
        }
    );
}

#[test]
fn errors_are_written_to_stderr() {
    let request = ConduitRequest::VM {
        code: "error(\"some error!\")".into(),
    };

    let result = Conduit::launch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(1),
            stdout: "".into(),
            stderr: "[string \"chunk\"]:1: some error!\nstacktrace:\n[C] function error\n[string \"chunk\"]:1\n\n".into()
        }
    );
}

#[test]
fn mix_of_stdout_and_stderr() {
    let request = ConduitRequest::VM {
        code: "print(5)\nerror(\"some error!\")".into(),
    };

    let result = Conduit::launch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(1),
            stdout: "5\n".into(),
            stderr: "[string \"chunk\"]:2: some error!\nstacktrace:\n[C] function error\n[string \"chunk\"]:2\n\n".into()
        }
    );
}

#[test]
fn print_some_null_bytes() {
    let request = ConduitRequest::VM {
        code: "print(\"\\0\")".into(),
    };

    let result = Conduit::launch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(0),
            stdout: "\u{0}\n".into(),
            stderr: "".into()
        }
    );
}
