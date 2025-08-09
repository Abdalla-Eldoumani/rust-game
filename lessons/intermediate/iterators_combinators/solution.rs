pub fn sum_first_five(nums: &[&str]) -> i32 {
    nums.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .take(5)
        .sum()
}






