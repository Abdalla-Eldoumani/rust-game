Goal: Use iterator adapters to transform and aggregate data.

Key ideas:
- `iter()` borrows elements; use `filter`, `map`, and `sum`.
- Avoid indexing loops; prefer combinators.

Example:
```rust
pub fn sum_squares_of_evens(nums: &[i32]) -> i32 {
    nums.iter().filter(|&&n| n % 2 == 0).map(|&n| n*n).sum()
}
```

Why this matters: Iterator chains are expressive and optimized by the compiler.




