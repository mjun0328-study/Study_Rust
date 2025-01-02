use std::cell::RefCell;

fn main() {
  let counter = RefCell::new(0);
  println!("counter: {}", counter.borrow()); // immutable reference. dropped after println

  let mut alias_counter = counter.borrow_mut(); // mutable reference
  *alias_counter += 1;
  drop(alias_counter);

  println!("counter: {}", counter.borrow());
}