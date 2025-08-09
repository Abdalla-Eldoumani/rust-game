#[test]
fn collects_from_workers() {
    let out = exercise_sandbox::fan_out_in(8);
    let mut expect: Vec<usize> = (0..8).collect();
    expect.sort();
    let mut got = out.clone();
    got.sort();
    assert_eq!(got, expect);
}


