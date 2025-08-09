#[test]
fn picks_longer() {
    let a = "alpha";
    let b = "beta123";
    assert_eq!(exercise_sandbox::longest(a, b), b);
}

#[test]
fn equal_len_prefers_first() {
    let a = "abcd";
    let b = "wxyz";
    assert_eq!(exercise_sandbox::longest(a, b), a);
}


