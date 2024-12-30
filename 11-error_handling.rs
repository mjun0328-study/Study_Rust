const CURRENT_YEAR: u16 = 2024;

fn get_age1(year: u16) -> Result<u16, String> {
  if year > CURRENT_YEAR {
    Err(String::from("Are you from the future?"))
  }else {
    Ok(CURRENT_YEAR - year + 1)
  }
}

fn get_age2(year: u16) -> Option<u16> {
  if year > CURRENT_YEAR {
    None
  }else {
    Some(CURRENT_YEAR - year + 1)
  }
}

fn check(year: u16) {
  match get_age1(year) {
    Ok(age) => println!("Your age is {}", age),
    Err(error) => println!("{}", error)
  }

  match get_age2(year) {
    Some(age) => println!("Your age is {}", age),
    None => println!("Are you from the future?")
  }
}

fn main() {
  check(2004);
  check(2040);
}