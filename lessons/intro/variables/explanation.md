Goal: Practice basic bindings and returning a value.

Key ideas:
- Use `let` to bind values. Mutability requires `let mut`.
- The function just needs to return 42.

Example:

```rust
pub fn answer() -> i32 {
    let x = 42;
    x
}
```

Why this matters: Variables are immutable by default in Rust, which reduces accidental state changes and data races.




