use conduit::{Conduit, ConduitRequest, Output};

#[test]
fn does_linting_work() {
    let request = ConduitRequest::Lint {
        code: "local _x = math.random() > 0.5 and nil or 5".into(),
    };

    let result = Conduit::launch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(1),
            stdout: "".into(),
            stderr: "MisleadingAndOr: The and-or expression always evaluates to the second alternative because \
                     the first alternative is nil; consider using if-then-else expression instead\n".into(),
        }
    )
}
