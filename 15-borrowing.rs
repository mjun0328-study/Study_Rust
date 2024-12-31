fn main() {
  let mut s = String::from("hello");

  let immutable_borrow = &s;
  println!("Immutable Borrowing String: {}", immutable_borrow);

  let mutable_borrow = &mut s;
  // let immutable_borrow = &s; fail - cannot have both mutable and immutable references at the same time
  println!("Mutable Borrowing String: {}", mutable_borrow);

  replace_string(mutable_borrow, "bye");
  println!("Replaced String: {}", s);
}

fn replace_string(s: &mut String, new_content: &str) {
  s.clear();
  s.push_str(new_content);
}