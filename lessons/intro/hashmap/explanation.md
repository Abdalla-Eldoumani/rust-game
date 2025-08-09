Goal: Count frequencies and resolve ties by first occurrence.

Key ideas:
- Use `HashMap<i32, usize>` and a separate `Vec<i32>` to track order.
- Return `None` for empty input.

Example idea:
1) Increment counts via `entry(n).or_insert(0)`
2) Push to order when first seen
3) Scan order to find the max count

Why this matters: HashMap patterns are foundational for data processing.




