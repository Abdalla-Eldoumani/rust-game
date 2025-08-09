pub fn sum_first(slice: &[i32], n: usize) -> i32 {
    let k = n.min(slice.len());
    slice[..k].iter().copied().sum()
}


