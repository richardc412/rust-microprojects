enum Shape {
    Circle(f64),          // radius
    Rectangle(f64, f64),  // width, height
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
      use std::f64::consts::PI;
       match self {
        Shape::Circle(r) => PI * r * r,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle { base, height } => 0.5 * base * height,
       }
    }
}

pub fn run() {
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { base: 5.0, height: 2.0 },
    ];
    for s in shapes {
        println!("Area = {:.2}", s.area());
    }
}