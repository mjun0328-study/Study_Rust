fn main() {
  // data type of array is [T; N] where T is element type and N is fixed length known at compiled
  let nums: [i32; 3] = [1, 2, 3];
  println!("{:?}", nums);
  println!("{}", nums[1]);
}