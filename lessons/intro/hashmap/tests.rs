#[test]
fn finds_mode() {
    let v = [3,1,2,3,2,3,2,2];
    assert_eq!(crate::most_frequent(&v), Some(2));
}

#[test]
fn empty_none() {
    let v: [i32;0] = [];
    assert_eq!(crate::most_frequent(&v), None);
}

#[test]
fn tie_returns_first_seen() {
    // 1 and 2 both appear 2 times; 1 appears first
    let v = [1,2,1,2];
    assert_eq!(crate::most_frequent(&v), Some(1));
}


