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
