fn main() {
  let mut vec = vec![1, 2, 3, 4, 5];

  // create an iterator that produces immutable references
  for element in vec.iter() {
    println!("Element: {}", element);
  }

  // create an iterator that produces mutable references
  for element in vec.iter_mut() {
    *element *= 2;
  }
  println!("New Vector: {:?}", vec);

  // create an iterator that takes ownership of elements
  let mut new_vec: Vec<i32> = Vec::new();
  for element in vec.into_iter() {
    new_vec.push(element / 2);
  }
  // println!("Another Vector: {:?}", vec); // vec is no longer alive
  println!("Another Vector: {:?}", new_vec);

  
  let mut numbers = vec![1, 2, 3, 4, 5].into_iter();

  // next returns each element in order, and returns None when all elements have been iterated
  while let Some(number) = numbers.next() {
    println!("Number: {}", number);
  }


  let numbers = vec![1, 2, 3, 4, 5];
  let doubled_numbers: Vec<_> = numbers.into_iter()
    .map(|x| x * 2)
    .collect(); // transform iterator elements into another collection type
  println!("{:?}", doubled_numbers);
}