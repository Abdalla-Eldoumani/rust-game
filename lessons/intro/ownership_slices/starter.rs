/// Return a slice of `arr` from `start` (inclusive) to `end` (exclusive).
/// Ensure indices are in range; otherwise return an empty slice.
pub fn subslice<'a>(arr: &'a [i32], start: usize, end: usize) -> &'a [i32] {
    if start <= end && end <= arr.len() { &arr[start..end] } else { &arr[0..0] }
}


