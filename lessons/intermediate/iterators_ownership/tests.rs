#[test]
fn filters_ascii_and_limits_to_5() {
    let v = vec![
        "Ann".to_string(),
        "Björk".to_string(), // non-ascii
        "Cal".to_string(),
        "Dee".to_string(),
        "Elle".to_string(),
        "Flo".to_string(),
        "Gia".to_string(),
    ];
    let got = crate::ascii_lengths_first5(v);
    assert_eq!(got, vec![3,3,3,4,3]);
}


