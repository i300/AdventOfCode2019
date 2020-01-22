use crate::days::Day;
use crate::computer::IntcodeComputer;
use crate::Result;
use crate::StringError;

pub struct Seven {
  filename: &'static str
}

impl Seven {
  pub fn new(filename: &'static str) -> Seven {
    Seven { filename }
  }
}

fn compute_output(memory: &mut Vec<i32>, phases: &Vec<i32>) -> Result<i32> {
  let mut last_output = 0;
  for phase in phases {
    // Setup computer with fresh copy of the program
    let mut computer = IntcodeComputer::new(memory);

    // Write the phase and the last output to the computer, then execute
    computer.write(last_output);
    computer.write(*phase);
    computer.execute()?;

    // Read from iostream until empty and write to last_output
    if let Some(output) = computer.read() {
      last_output = output;
    } else {
      return Err(Box::new(StringError::from("Computer did not output value after execution")))
    }

    println!("{}", last_output);
  }

  Ok(last_output)
}

impl Day for Seven {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let memory: Vec<i32> = match contents.split(",").map(|s| s.parse::<i32>()).collect() {
      Ok(s) => s,
      Err(e) => return Err(Box::new(e))
    };

    let input = vec![0,1,2,3,4];
    input.perm

    Ok("".to_string())
  }
}