const PI: f32 = std::f32::consts::PI;

trait Area {
  fn get_area(&self) -> f32;
}

struct Point3D(f32, f32, f32);

enum Shape { // Tuple Enum Variant
  Circle { radius: f32 },
  Rectangle { width: f32, height: f32 },
  Triangle { base: f32, height: f32 }
}

impl Area for Shape {
  fn get_area(&self) -> f32 {
    match self {
      Shape::Circle { radius } => PI * radius * radius,
      Shape::Rectangle { width, height } => width * height,
      Shape::Triangle { base, height } => base * height / 2.0
    }
  }
}

fn main() {
  let point = Point3D(1.0, 2.4, 3.8);
  println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);

  let circle = Shape::Circle { radius: 10.0 };
  let rectangle = Shape::Rectangle { width: 4.0, height: 1.5 };
  let triangle = Shape::Triangle { base: 2.0, height: 6.0 };

  println!("Circle Area: {}", circle.get_area());
  println!("Rectangle Area: {}", rectangle.get_area());
  println!("Triangle Area: {}", triangle.get_area());
}