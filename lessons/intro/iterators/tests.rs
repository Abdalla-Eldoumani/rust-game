#[test]
fn sums_even_squares() {
    let v = [1,2,3,4,5,6];
    // evens: 2,4,6 => squares: 4,16,36 => sum: 56
    assert_eq!(crate::sum_squares_of_evens(&v), 56);
}


