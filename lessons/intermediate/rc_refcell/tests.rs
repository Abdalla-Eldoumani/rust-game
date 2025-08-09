#[test]
fn pushes_from_multiple_closures() {
    let v = crate::shared_push(3);
    let mut expect = vec![1,2,3,1,2,3];
    expect.sort();
    let mut got = v.clone();
    got.sort();
    assert_eq!(got, expect);
}


