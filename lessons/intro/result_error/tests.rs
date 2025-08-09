#[test]
fn parse_ok_and_err() {
    assert_eq!(crate::parse_i32("42"), Ok(42));
    assert_eq!(crate::parse_i32("nope"), Err("nope".to_string()));
}


