pub enum Shape {
    Circle(f32),
    Square(f32),
}

pub fn area(s: Shape) -> f32 {
    match s {
        Shape::Circle(r) => std::f32::consts::PI * r * r,
        Shape::Square(a) => a * a,
    }
}


