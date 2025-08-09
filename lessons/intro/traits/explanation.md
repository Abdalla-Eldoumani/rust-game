Goal: Define a trait and implement it for a type.

Key ideas:
- Traits specify shared behavior; types implement traits.
- Methods take `&self` and return owned values when needed.

Example:
```rust
pub struct Person { pub name: String }
pub trait Describable { fn describe(&self) -> String; }
impl Describable for Person {
    fn describe(&self) -> String { format!("Person: {}", self.name) }
}
```

Why this matters: Traits enable abstraction without inheritance.




