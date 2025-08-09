Goal: Collect results from an iterator, short-circuiting on error.

Key ideas:
- Use `map` to parse, then `collect::<Result<Vec<_>, _>>()` to transpose.
- The first error halts collection and is returned.

Example:
```rust
pub fn parse_all(nums: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    nums.iter().map(|s| s.parse::<i32>()).collect()
}
```

Why this matters: Leveraging `collect` on `Result` simplifies error-aware pipelines.




