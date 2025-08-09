use crate::Shape::*;

#[test]
fn area_works() {
    let c = Circle(1.0);
    let s = Square(2.0);
    let ca = crate::area(c);
    let sa = crate::area(s);
    let expected_c = std::f32::consts::PI;
    let expected_s = 4.0f32;
    assert!((ca - expected_c).abs() < 1e-3, "circle area: expected ~{expected_c}, got {ca}");
    assert!((sa - expected_s).abs() < 1e-6, "square area: expected {expected_s}, got {sa}");
}


