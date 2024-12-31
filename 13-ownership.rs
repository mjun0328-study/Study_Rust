fn take_ownership(s: String) {
  // ownership of s moves into this function and is dropped when function ends
  println!("{}", s);
}

fn take_and_give_ownership(s: String) -> String {
  // ownership of s moves into this function and is returned when function ends
  println!("{}", s);
  return s;
}


fn main() {
  let s1 = String::from("hello");
  let s2 = s1; // ownership of s1 moves into s2
  // println!("{}", s1); fail
  println!("{}", s2);

  take_ownership(s2);
  // println!("{}", s2); fail


  let s1 = String::from("hello");
  let s2 = take_and_give_ownership(s1); // ownership of s1 moves into function and then into s2
  // println!("{}", s1); fail
  println!("{}", s2);


  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("{}", s1);
  println!("{}", s2);
}