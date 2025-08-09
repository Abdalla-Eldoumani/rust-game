pub fn ascii_lengths_first5(names: Vec<String>) -> Vec<usize> {
    names
        .into_iter()
        .filter(|s| s.is_ascii())
        .map(|s| s.len())
        .take(5)
        .collect()
}


