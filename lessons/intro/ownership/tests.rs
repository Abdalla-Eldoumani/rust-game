#[test]
fn greet_does_not_move() {
    // The intended solution changes the signature to &str
    // This test checks behavior, not signature type directly.
    let name = String::from("Ferris");
    let out = crate::greet_name(name.as_str());
    assert_eq!(out, "Hello, Ferris!");
    // name is still usable here because we only borrowed
    assert_eq!(name, "Ferris");
}


