use conduit::{Conduit, ConduitRequest, Output};

#[test]
fn does_the_type_checker_work() {
    let request = ConduitRequest::Check {
        code: "local x: string = 5".into(),
    };

    let result = Conduit::dispatch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(1),
            stdout: "".into(),
            stderr: "TypeError: Type 'number' could not be converted into 'string'\n".into(),
        }
    )
}
