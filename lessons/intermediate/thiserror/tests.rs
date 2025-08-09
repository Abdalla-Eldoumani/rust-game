#[test]
fn parse_maps_error() {
    let e = crate::parse_id("x").unwrap_err();
    let _ = e; // type compiles and error is returned
}


