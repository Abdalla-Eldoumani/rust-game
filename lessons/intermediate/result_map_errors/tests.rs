#[test]
fn parses_ok() {
    let out = exercise_sandbox::parse_all(&["1","2","10"]).unwrap();
    assert_eq!(out, vec![1,2,10]);
}

#[test]
fn fails_on_bad() {
    let err = exercise_sandbox::parse_all(&["1","x","3"]).unwrap_err();
    let _ = err; // just ensure it's an error
}


