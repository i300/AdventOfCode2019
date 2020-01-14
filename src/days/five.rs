use crate::days::Day;
use crate::computer::IntcodeComputer;
use crate::Result;

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
    let memory: Vec<i32> = match contents.split(",").map(|s| s.parse::<i32>()).collect() {
      Ok(s) => s,
      Err(e) => return Err(Box::new(e))
    };
    // Part 1
    let mut computer = IntcodeComputer::new(&memory);
    computer.write(5); // write 1 for pt 1, write 5 for pt2
    computer.execute()?;
    Ok(computer.read().to_string())
  }
}