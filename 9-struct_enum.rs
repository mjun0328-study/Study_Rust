trait Greet { // define trait
  fn greet(&self) -> String;
}

struct Address {
  country: String,
  city: String
}

struct Person {
  name: String,
  address: Address // nested structure
}

impl Greet for Person { // implement defined trait for structure
  fn greet(&self) -> String {
    format!("Hello, {}!", self.name)
  }
}

enum Animal {
  Dog,
  Cat
}

impl Greet for Animal { // implement defined trait for enumeration
  fn greet(&self) -> String {
    match self {
      Animal::Dog => String::from("bow-bow"),
      Animal::Cat => String::from("meow")
    }
  }
}

fn main() {
  let person = Person {
    name: String::from("Alice"),
    address: Address {
      country: String::from("South Korea"),
      city: String::from("Seoul")
    }
  };
  let dog = Animal::Dog;
  let cat = Animal::Cat;

  println!("{} lived in {}, {}", person.name, person.address.city, person.address.country);
  println!("{}: {}", person.name, person.greet());
  println!("{}'s puppy: {}", person.name, dog.greet());
  println!("{}'s kitty: {}", person.name, cat.greet());
}