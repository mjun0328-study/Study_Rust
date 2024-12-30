fn main() {
  // while
  let mut a = 2;
  while a < 10 {
    let mut b = 1;
    while b < 10 {
      print!("{}*{}={}\t", a, b, a*b);
      b += 1;
    }
    println!("");
    a += 1;
  }

  println!("");

  // for
  for a in 2..10 { // range(2, 10)
    for b in 1..=9 { // range(1, 9 + 1)
      print!("{}*{}={}\t", a, b, a*b);
    }
    println!("");
  }

  println!("");

  // loop
  let mut a = 2;
  loop {
    let mut b = 1;
    loop {
      print!("{}*{}={}\t", a, b, a*b);

      if b >= 9 {
        break;
      }else {
        b += 1;
      }
    }
    println!("");

    if a >= 9 {
      break;
    }else {
      a += 1;
    }
  }

  println!("");

  // find prime numbers with labeled for & loop
  print!("소수: ");
  let mut number = 2;

  'search: loop {
    let mut is_prime = true;

    'check_prime: for i in 2..number {
      if number & i == 0 {
        is_prime = false;
        break 'check_prime;
      }
    }

    if is_prime {
      print!("{} ", number);
    }

    number += 1;
    if number > 20 {
      break 'search;
    }
  }
}