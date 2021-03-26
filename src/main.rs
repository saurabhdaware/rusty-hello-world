use time::util;
use std::io;

fn main() {
  println!("Enter year:");
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("could not read a valid input");
  let year:i32 = n.trim().parse().expect("invalid input");
  println!("There are {} days in year {}", util::days_in_year(year), year);
}