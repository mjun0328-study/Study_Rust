fn main() {
  let name = "RUST";
  let age: u16 = 3;

  let name = "rust";
  println!("Welcome, {}!", name);

  let mut level = 0;
  println!("Now Level is {}", level);
  level = 10;
  println!("Now Level is {}", level);
  level = 25;
  println!("Now Level is {}", level);

  let your_age = match age {
    1 => "one year old",
    2 => "two years old",
    3 => "three years old",
    _ => "too old..."
  };
  println!("You are {}", your_age);

  let is_completed = if level > 30 {
    true
  }else {
    false
  };
  println!("Rust Complete: {}", is_completed)
}