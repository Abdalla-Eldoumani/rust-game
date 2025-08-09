pub fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| s.to_string())
}


