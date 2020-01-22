use crate::days::Day;
use crate::computer::IntcodeComputer;
use crate::Result;
use crate::StringError;

pub struct Five {
    filename: &'static str
}

impl Five {
  pub fn new(filename: &'static str) -> Five {
    Five { filename }
  }
}

impl Day for Five {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let mut memory: Vec<i32> = match contents.split(",").map(|s| s.parse::<i32>()).collect() {
      Ok(s) => s,
      Err(e) => return Err(Box::new(e))
    };
    // Part 1
    let mut computer = IntcodeComputer::new(&mut memory);
    computer.write(5); // write 1 for pt 1, write 5 for pt2
    computer.execute()?;
    if let Some(result) = computer.read() {
      Ok(result.to_string())
    } else {
      Err(Box::new(StringError::new("Computer's iostream was empty".to_string())))
    }
  }
}