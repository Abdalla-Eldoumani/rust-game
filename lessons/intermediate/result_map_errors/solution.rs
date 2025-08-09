pub fn parse_all(nums: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    nums.iter().map(|s| s.parse::<i32>()).collect()
}


