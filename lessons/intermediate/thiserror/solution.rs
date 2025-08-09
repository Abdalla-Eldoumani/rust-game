use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

pub fn parse_id(s: &str) -> Result<i32, MyError> {
    Ok(s.parse::<i32>()?)
}


