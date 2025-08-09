// Define a custom error type using thiserror.
// Provide two variants: NotFound(String) and Parse(std::num::ParseIntError).
// Implement a function parse_id(s: &str) -> Result<i32, MyError>
// that maps the parse error into MyError::Parse.
pub fn parse_id(_s: &str) -> Result<i32, String> {
    Err("todo".into())
}


