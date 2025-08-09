#[test]
fn counts_correctly() {
    let n = exercise_sandbox::parallel_count(8, 10_000) as i64;
    assert_eq!(n, 8 * 10_000);
}


