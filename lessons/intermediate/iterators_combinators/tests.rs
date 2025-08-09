#[test]
fn sums_first_five_valids() {
    // valid parsed numbers: 1,2,3,4,5 => sum 15 (only first 5 are taken)
    let xs = ["1","x","2","3","4","y","5","6"]; 
    assert_eq!(crate::sum_first_five(&xs), 15);
}






