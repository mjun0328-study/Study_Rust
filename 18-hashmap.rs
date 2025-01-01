use std::collections::HashMap;


fn main() {
  let mut scores: HashMap<String, u32> = HashMap::new(); // create empty hashmap


  // insert key-value
  scores.insert(String::from("Alice"), 50);
  scores.insert(String::from("Bob"), 86);
  scores.insert(String::from("Charlie"), 62);
  scores.insert(String::from("Dave"), 62);


  // update key-value
  scores.insert(String::from("Charlie"), 72);


  // remove key-value
  scores.remove("Dave");


  // get key-value
  if let Some(score) = scores.get("Alice") {
    println!("Alice's score: {}", score);
  }else {
    println!("Alice is not here!");
  }


  // iterate hashmap
  for (key, value) in &scores {
    println!("{}'s score: {}", key, value);
  }


  // check if key exists
  if scores.contains_key("Erin") {
    println!("There is Erin!");
  }else {
    println!("Erin is not here :(");
  }


  // get all keys in hashmap
  for key in scores.keys() {
    println!("name: {}", key);
  }


  // get all values in hashmap
  for value in scores.values() {
    println!("score: {}", value);
  }
}