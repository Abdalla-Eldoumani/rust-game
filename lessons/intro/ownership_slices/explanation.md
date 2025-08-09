Goal: Practice borrowing slices without owning data.

Key ideas:
- Borrow with `&[T]` and return subslices with range syntax.
- Ensure indices are valid to avoid panics.

Example:
```rust
pub fn subslice<'a>(arr: &'a [i32], start: usize, end: usize) -> &'a [i32] {
    if start <= end && end <= arr.len() { &arr[start..end] } else { &arr[0..0] }
}
```

Why this matters: Slices provide zero-copy views into contiguous data.


