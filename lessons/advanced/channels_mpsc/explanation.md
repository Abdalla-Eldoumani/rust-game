Goal: Coordinate threads via message passing.

Key ideas:
- Use `std::sync::mpsc::{channel, Sender}` for communication.
- Clone the sender for each worker and send messages.
- Drop the original sender in the parent so the receiver ends when workers finish.

Example:
```rust
use std::sync::mpsc;
use std::thread;

pub fn fan_out_in(workers: usize) -> Vec<usize> {
    let (tx, rx) = mpsc::channel();
    for i in 0..workers {
        let txc = tx.clone();
        thread::spawn(move || { let _ = txc.send(i); });
    }
    drop(tx);
    let mut out: Vec<usize> = rx.into_iter().collect();
    out.sort();
    out
}
```

Why this matters: Channels enable clear ownership and synchronization boundaries.


