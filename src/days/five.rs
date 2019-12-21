use crate::days::Day;
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
    let lines = contents.lines();

    let mut sum = 0;
    for line in lines {
      let value = line.parse::<i64>()?;
      let mut result = value / 3 - 2;
      
      let mut new_mass = result / 3 - 2;
      loop {
        result += new_mass;
        new_mass = new_mass / 3 - 2;
        if new_mass <= 0 {
          break;
        }
      }
      
      sum += result;
    }

    Ok(sum.to_string())
  }
}