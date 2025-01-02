enum List {
  Cons(i32, Box<List>),
  Nil
}

use crate::List::{Cons, Nil};

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  print_list(list);
}

fn print_list(list: List) {
  match list {
    Cons(value, next) => {
      println!("number: {}", value);
      print_list(*next);
    },
    Nil => println!("List is empty")
  }
}