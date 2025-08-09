#[test]
fn greets_with_emoji() {
    let s = crate::greet("世界");
    assert_eq!(s, "Hello, 世界! 🌟");
}






