use conduit::{Conduit, ConduitRequest, Output};

#[test]
fn ascribe_types_from_type_inference() {
    let request = ConduitRequest::Hydrate {
        code: "--!strict\nlocal x = 5".into(),
    };

    let result = Conduit::dispatch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(0),
            stdout: "\nlocal x:number=5\n".into(),
            stderr: "".into(),
        }
    )
}
