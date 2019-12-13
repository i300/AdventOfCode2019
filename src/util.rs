use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn read_file(filename: &str) -> std::io::Result<String> {
  let file = File::open(filename)?;
  let mut reader = BufReader::new(file);
  let mut contents = String::new();
  reader.read_to_string(&mut contents)?;
  Ok(contents)
}

#[derive(Debug)]
pub struct StringError(String);

impl StringError {
  pub fn new(error: String) -> StringError {
    StringError(error)
  }
}

impl fmt::Display for StringError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for StringError {}