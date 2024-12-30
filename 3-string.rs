fn main() {
  // str
  let my_literal: &'static str = "안녕하세요!";

  // &str
  let full_name = "홍길동";
  let first_name: &str = &full_name[3..]; // 3 per each korean char.
  println!("{}의 이름은 {}입니다", full_name, first_name);

  // String
  let mut greeting = String::from("Hello, ");
  greeting.push_str("rust!");
  println!("{}", greeting);

  // to_owned(): &str이나 다른 불변 타입의 참조에서 String 타입으로 변경할 때
  let my_str = "안녕하세요!";
  let my_string = my_str.to_owned();
  println!("{}", my_string);

  // to_string(): 숫자나 사용자 정의 타입에서 String 타입으로 변경할 때
  let my_number = 42;
  let my_number_string = my_number.to_string(); // 42 -> "42"
  println!("{}", my_number_string);
}