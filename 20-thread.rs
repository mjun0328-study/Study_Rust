use std::thread;
use std::time::Duration;

fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("Spawned Thread: {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("Main Thread: {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();


  let numbers = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("from 1 to 3: {:?}", numbers);
  });
  handle.join().unwrap();
}