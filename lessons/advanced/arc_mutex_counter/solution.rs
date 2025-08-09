use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_count(threads: usize, iters: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0i32));
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..iters {
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles { let _ = h.join(); }
    *counter.lock().unwrap()
}


