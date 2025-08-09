#[test]
fn sums_capped() {
    let v = [1,2,3,4,5];
    assert_eq!(crate::sum_first(&v, 3), 6);
    assert_eq!(crate::sum_first(&v, 99), 15);
}


