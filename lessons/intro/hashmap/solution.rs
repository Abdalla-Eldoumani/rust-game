use std::collections::HashMap;

pub fn most_frequent(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() { return None; }
    let mut counts: HashMap<i32, usize> = HashMap::new();
    let mut order: Vec<i32> = Vec::new();
    for &n in nums {
        let e = counts.entry(n).or_insert(0);
        if *e == 0 { order.push(n); }
        *e += 1;
    }
    let mut best: Option<(i32, usize)> = None;
    for &n in &order {
        let c = counts.get(&n).copied().unwrap_or(0);
        match best {
            None => best = Some((n, c)),
            Some((_, bc)) if c > bc => best = Some((n, c)),
            _ => {}
        }
    }
    best.map(|(n, _)| n)
}


