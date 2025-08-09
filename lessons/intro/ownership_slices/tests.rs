#[test]
fn returns_subslice() {
    let v = [10,20,30,40,50];
    assert_eq!(crate::subslice(&v, 1, 4), &[20,30,40]);
}

#[test]
fn out_of_bounds_empty() {
    let v = [1,2,3];
    assert_eq!(crate::subslice(&v, 2, 5), &[]);
}


