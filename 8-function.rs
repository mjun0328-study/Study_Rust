fn add(x: i32, y: i32) -> i32 {
  return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
  x - y // you can return expression without 'return' keyword and ';'
}

fn swap(x: i32, y: i32) -> (i32, i32) {
  return (y, x) // return mulitple values with tuple
}

fn return_nothing() -> () {
  return ();
}

fn return_nothing2() {
  // nothing here!
}

fn main() {
  let a = 42;
  let b = 13;
  println!("42 + 13 = {}", add(a, b));
  println!("42 - 13 = {}", subtract(a, b));

  let (new_a, new_b) = swap(a, b);
  let result = swap(a, b);
  println!("a = {}, b = {}", new_a, new_b);
  println!("a = {}, b = {}", result.0, result.1);

  println!("Nothing: {:?}", return_nothing());
  println!("Nothing: {:?}", return_nothing2());
}