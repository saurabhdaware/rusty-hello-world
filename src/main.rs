// use time::util;
// use std::io;

fn main() {
  let mut num = 0;
  println!("EVEN NUMBERSS!!");
  loop {
    println!("{}", num);
    if num >= 10 {
      break;
    }
    num += 2;
  }

  println!("\nFirst 10 numbers!!");
  for z in 1..11 {
    println!("FOR: {}", z);
  }

}