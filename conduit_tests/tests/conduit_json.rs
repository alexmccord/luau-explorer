use conduit::{Conduit, ConduitRequest, Output};

#[test]
fn encode_some_code() {
    let request = ConduitRequest::Json {
        code: "local x: string = 5".into(),
    };

    let result = Conduit::dispatch(request).unwrap();

    assert_eq!(
        result,
        Output {
            status: Some(0),
            stdout: "{\
                      \"type\":\"AstStatBlock\",\"location\":\"0,0 - 0,19\",\"body\":[{\"type\":\"AstStatLocal\",\
                      \"location\":\"0,0 - 0,19\",\"vars\":[{\"type\":{\"type\":\"AstTypeReference\",\
                      \"location\":\"0,9 - 0,15\",\"name\":\"string\",\"parameters\":[]},\"name\":\"x\",\
                      \"location\":\"0,6 - 0,7\"}],\"values\":[{\"type\":\"AstExprConstantNumber\",\
                      \"location\":\"0,18 - 0,19\",\"value\":5}]}]\
                     }\n".into(),
            stderr: "".into(),
        }
    )
}
