/// Return a reference to the longer of two string slices.
///
/// Fill in the correct lifetime annotations.
pub fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}


