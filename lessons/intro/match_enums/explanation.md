Goal: Use `match` to handle enum variants.

Key ideas:
- Cover all variants in the `match` to be exhaustive.
- Use precise math (e.g., `std::f32::consts::PI`) for numeric tasks.

Example:
```rust
pub enum Shape { Circle(f32), Square(f32) }
pub fn area(s: Shape) -> f32 {
    match s {
        Shape::Circle(r) => std::f32::consts::PI * r * r,
        Shape::Square(a) => a * a,
    }
}
```

Why this matters: Pattern matching is a core Rust feature for expressive, safe control flow.




