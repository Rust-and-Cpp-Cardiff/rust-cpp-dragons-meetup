use std::thread;

// fn main() {
//     let str = "Hello world";
//     let handle = thread::spawn(|| {
//         println!("str = {}", str);
//     });
//     handle.join();
// }

fn main() {
  let str = "Hello world";
  thread::scope(|s| {
    s.spawn(|| {
      println!("str = {}", str);
    });
  });
}
