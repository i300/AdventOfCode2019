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
    let mut computer = IntcodeComputer::new(&mut memory);
    // computer.write(1); // Part 1
    computer.write(5); // Part 2
    computer.execute()?;

    // Get result, which is the final diagnostic code
    let mut last_result = 0;
    while let Some(result) = computer.read() {
      if result != 0 && !computer.is_io_empty() {
        return Err(Box::new(StringError::new("Diagnostic test failed".to_string())))
      }
      last_result = result;
    }
    Ok(last_result.to_string())
  }
}