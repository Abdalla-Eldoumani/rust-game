use std::f64::consts::PI;

trait TestShape { fn area(&self) -> f64; }
struct Circle { r: f64 }
struct Rect { w: f64, h: f64 }
impl TestShape for Circle { fn area(&self) -> f64 { PI * self.r * self.r } }
impl TestShape for Rect { fn area(&self) -> f64 { self.w * self.h } }

#[test]
fn sums_dyn_areas() {
    struct Wrap(Box<dyn TestShape>);
    let shapes: Vec<_> = vec![
        Wrap(Box::new(Circle{ r: 1.0 })),
        Wrap(Box::new(Rect{ w: 2.0, h: 3.0 })),
    ];
    let sum: f64 = shapes.iter().map(|w| w.0.area()).sum();
    let expected = PI + 6.0;
    assert!((sum - expected).abs() < 1e-9);
}


