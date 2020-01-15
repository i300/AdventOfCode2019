use crate::days::Day;
use crate::Result;
use crate::StringError;

pub struct Six {
    filename: &'static str
}

impl Six {
  pub fn new(filename: &'static str) -> Six {
    Six { filename }
  }
}

impl Day for Six {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let lines = contents.lines();
    
    for line in lines {
        let orbitPair: Vec<&str> = line.split(")").collect();
        let oribed = match orbitPair.get(0) {
            Some(v) => *v,
            None => return Err(Box::new(StringError::new("Orbit pair does not contain orbited.".to_string())))
        };

        let orbiter = match orbitPair.get(1) {
            Some(v) => *v,
            None => return Err(Box::new(StringError::new("Orbit pair does not contain orbiter.".to_string())))
        };
    }

    Ok(String::from("123"))
  }
}