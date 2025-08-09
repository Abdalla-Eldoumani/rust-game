/// Define a trait Shape with fn area(&self) -> f64.
/// Implement for Circle {r:f64} and Rect {w:f64,h:f64}.
/// Implement sum_areas(shapes: &[Box<dyn Shape>]) -> f64.
pub trait Shape { fn area(&self) -> f64; }

pub fn sum_areas(_shapes: &[Box<dyn Shape>]) -> f64 {
    0.0
}


