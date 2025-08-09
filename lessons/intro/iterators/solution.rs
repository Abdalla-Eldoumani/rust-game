pub fn sum_squares_of_evens(nums: &[i32]) -> i32 {
    nums.iter().filter(|&&n| n % 2 == 0).map(|&n| n * n).sum()
}






