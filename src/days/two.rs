use crate::days::Day;
use crate::computer::IntcodeComputer;
use crate::Result;

/* -- Main Type -- */
pub struct Two {
  filename: &'static str
}

impl Two {
  pub fn new(filename: &'static str) -> Two {
    Two { filename }
  }
}

impl Day for Two {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let memory: Vec<i32> = match contents.split(",").map(|s| s.parse::<i32>()).collect() {
      Ok(s) => s,
      Err(e) => return Err(Box::new(e))
    };
    // Part 1
    // let mut computer = IntcodeComputer::new(&memory);
    // computer.execute()?;
    // Ok(computer.get_value(0)?.to_string())
    // Part 2
    let target_value = 19690720;
    for noun in 0..100 {
      for verb in 0..100 {
        let mut cloned_memory = memory.clone();
        let mut computer = IntcodeComputer::new(&mut cloned_memory);
        computer.set_value(1, noun)?;
        computer.set_value(2, verb)?;
        computer.execute()?;
        if computer.get_value(0)? == target_value {
          return Ok((100 * noun + verb).to_string());
        }
      }
    }
    Ok("No value found.".to_string())
  }
}