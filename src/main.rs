use std::time::{SystemTime};
use std::fs;
use std::path::Path;


fn main() {
  let now = SystemTime::now();

  let path = Path::new("./src/hello.md");

  let data = fs::read_to_string(path).expect("Things didn't go well while reading");
  println!("{}", data);

  match now.elapsed() {
    Ok(elapsed) => {
      println!("{}", elapsed.as_millis());
    }
    Err(e) => {
        // an error occurred!
        println!("Error: {:?}", e);
    }
  }
}