fn main() {
  // create vector
  let mut vec1: Vec<i32> = Vec::new();
  let mut vec2 = vec![4, 4, 3, 2, 1];

  
  // modify vector
  vec2[0] = 5;
  vec2.push(0);


  // get element by indexing
  let first_element = vec2[0];
  println!("Countdown from {}", first_element);


  // iterate over gvector elements
  for element in vec2.iter() {
    println!("{}", element);
  }


  // slice vector
  let slice = &vec2[1..=3];
  println!("{:?}", slice);


  // resize and manage capcity
  vec1.reserve(10);  // reserve space for 10 elements
  vec1.shrink_to_fit();  // shrink capacity to fit current size
  vec1.resize(5, 0);  // resize vector to length 5, filling with zeros
  println!("{:?}", vec1);


  if let Some(last_element) = vec2.pop() {
    println!("Last Element: {}", last_element);
  }
}