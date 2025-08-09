Goal: Return `Result` and map errors.

Key ideas:
- Use `s.parse::<i32>()` which returns `Result<i32, ParseIntError>`.
- Convert the error with `.map_err(|_| s.to_string())`.

Example:
```rust
pub fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| s.to_string())
}
```

Why this matters: Idiomatic error handling is explicit and composable in Rust.




