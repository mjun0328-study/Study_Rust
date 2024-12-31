fn main() {
  let maybe_name = Some(String::from("Alice"));

  match maybe_name {
    Some(ref n) => println!("Hello, {}", n), // ref keyword prevents the ownership of maybe_name from being dropped after match
    _ => println!("Who are you?")
  }

  println!("Hello again, {}", maybe_name.unwrap()); // ownership of maybe_name is still alive
}