use std::error::Error;

// how to make a function
pub fn greet_int(x: i64) {
  println!("Hello to number {}.", x);
}

// how to make a string function
pub fn greet_str(x: &str) {
  println!("Hello to {}.", x);
}

pub fn return_str(x: &str) -> String {
  format!("Get back {}.", x)
}

// using enum
#[derive(Debug)]
pub enum IsTrue {
  True(i64),
  False,
}

pub fn process() -> Result<String, Box<dyn Error>> {
  Ok(String::from("Yay!"))
}
