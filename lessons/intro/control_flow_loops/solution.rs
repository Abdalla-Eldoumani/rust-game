pub fn sum_until(xs: &[i32], limit: i32) -> i32 {
    let mut sum = 0;
    for &x in xs {
        if x < 0 { continue; }
        if sum + x > limit { break; }
        sum += x;
    }
    sum
}


