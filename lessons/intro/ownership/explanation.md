Goal: Borrow instead of move; avoid taking ownership unnecessarily.

Key ideas:
- Prefer `&str` over `String` when you only need to read the string.
- Borrowing keeps the original `String` usable after the call.

Example:
```rust
pub fn greet_name(name: &str) -> String {
    format!("Hello, {name}!")
}
```

Why this matters: Understanding ownership and borrowing is essential for safe, efficient Rust.




