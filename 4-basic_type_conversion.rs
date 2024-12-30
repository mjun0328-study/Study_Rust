fn main() {
  let a = 13u8; // 13 unsigned int with 1 byte
  let b = 7u32; // 7 unsigned int with 4 bytes
  let c = a as u32 + b;
  println!("{} + {} = {}", a, b, c);

  let t = true;
  println!("True is {}", t as u8);

  let my_string = "42";
  let my_integer = my_string.parse::<i32>().unwrap();
  println!("'42' is {}", my_integer);
}