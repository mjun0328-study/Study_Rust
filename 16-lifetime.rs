// lifetime for structure
struct TwoString<'a> {
  str_a: &'a str,
  str_b: &'a str
}

// lifetime for function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  }else {
    y
  }
}

fn main() {
  let s1 = String::from("Alice");
  let s2 = String::from("Bob");

  let two_string = TwoString{
    str_a: &s1,
    str_b: &s2
  };

  let result = longest(two_string.str_a, two_string.str_b);
  println!("{}", result);

  /* Error
  {
    let s1 = String::from("Alice");
    let result = "";
    {
      let s2 = String::from("Bob");
      result = longest(&s1, &s2); fail - s2 will be dropped at the end of this scope, but result needs to live longer
    }
    println!("{}", result);
  }
  */
}