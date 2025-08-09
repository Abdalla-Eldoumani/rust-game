/// Spawn `workers` threads, each sending its index on an mpsc channel.
/// Collect all messages and return them sorted.
pub fn fan_out_in(workers: usize) -> Vec<usize> {
    // Starter placeholder
    (0..workers).collect()
}


