Goal: Tie output lifetime to inputs to avoid dangling references.

Key ideas:
- Annotate both inputs and the output with the same lifetime `'a`.
- The compiler ensures returned reference lives as long as the shorter input.

Example:
```rust
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Why this matters: Lifetimes make reference validity explicit, preventing use-after-free at compile time.




