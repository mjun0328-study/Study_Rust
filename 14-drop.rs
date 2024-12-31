struct CustomStruct {
  data: String
}

impl Drop for CustomStruct {
  fn drop(&mut self) {
    println!("Dropping CustomStruct with data: {}", self.data);
  }
}

fn main() {
  let custom = CustomStruct{
    data: String::from("Hello, World!")
  };

  println!("Created CustomStruct with data: {}", custom.data);
}