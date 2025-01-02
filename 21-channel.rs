use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

  let tx1 = tx.clone();
  thread::spawn(move || {
    let msgs = vec!["hello", "from", "the", "thread"];

    for msg in msgs {
      tx1.send(format!("A: {}", msg)).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let msgs = vec!["more", "message", "for", "you"];

    for msg in msgs {
      tx.send(format!("B: {}", msg)).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  loop {
    match rx.recv() {
      Ok(received) => println!("{}", received),
      Err(_) => break,
    }
  }

  // same as above loop - iterating over received messages
  // for received in rx {
  //   println!("{}", received);
  // }
}