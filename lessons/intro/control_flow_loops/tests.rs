#[test]
fn sums_with_break_and_continue() {
    let xs = [1,-5,3,4,10];
    // 1 + 3 + 4 = 8; next 10 would exceed 9 so stop
    assert_eq!(crate::sum_until(&xs, 9), 8);
}


