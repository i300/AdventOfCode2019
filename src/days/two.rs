use crate::days::Day;
use crate::StringError;
use crate::Result;

/* -- Main Type -- */
pub struct Two {
  filename: &'static str
}

fn get_value(memory: &Vec<u32>, ptr: usize) -> Result<u32> {
  match memory.get(ptr) {
    Some(v) => Ok(*v),
    None => Err(Box::new(StringError::new("Tried to read value out of bounds.".to_string())))
  }
}

fn set_value(memory: &mut Vec<u32>, ptr: usize, new: u32) -> Result<()> {
  if let Some(v) = memory.get_mut(ptr) {
    *v = new;
    Ok(())
  } else {
    return Err(Box::new(StringError::new("Tried to set out-of-bounds value.".to_string())));
  }
}

impl Two {
  pub fn new(filename: &'static str) -> Two {
    Two { filename }
  }

  fn add(&self, memory: &mut Vec<u32>, ptr: usize) -> Result<()> {
    let a_addr = get_value(memory, ptr + 1)?;
    let b_addr = get_value(memory, ptr + 2)?;
    let a = get_value(memory, a_addr as usize)?;
    let b = get_value(memory, b_addr as usize)?;
    let write_addr = get_value(memory, ptr + 3)?;
    set_value(memory, write_addr as usize, a + b)?;

    Ok(())
  }

  fn mul(&self, memory: &mut Vec<u32>, ptr: usize) -> Result<()> {
    let a_addr = get_value(memory, ptr + 1)?;
    let b_addr = get_value(memory, ptr + 2)?;
    let a = get_value(memory, a_addr as usize)?;
    let b = get_value(memory, b_addr as usize)?;
    let write_addr = get_value(memory, ptr + 3)?;
    set_value(memory, write_addr as usize, a * b)?;

    Ok(())
  }

  fn execute(&self, memory: &mut Vec<u32>) -> Result<()> {
    let mut ptr = 0;
    loop {
      if ptr >= memory.len() {
        return Err(Box::new(StringError::new("Stack pointer pointed out of bounds.".to_string())));
      }
      let code = memory.get(ptr);
      match code {
        Some(1) => self.add(memory, ptr)?,
        Some(2) => self.mul(memory, ptr)?,
        Some(99) => break,
        Some(op) => return Err(Box::new(StringError::new(format!("Invalid opcode found: {}.", op)))),
        None => return Err(Box::new(StringError::new("Accessed an out-of-bounds index".to_string()))),
      };
      ptr += 4;
    }
    Ok(())
  }
}

impl Day for Two {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let memory: Vec<u32> = match contents.split(",").map(|s| s.parse::<u32>()).collect() {
      Ok(s) => s,
      Err(e) => return Err(Box::new(e))
    };
    // Part 1
    // self.execute(&mut memory)?;
    // Ok(memory[0].to_string())
    // Part 2
    let target_value = 19690720;
    for noun in 0..100 {
      for verb in 0..100 {
        let mut instance_memory = memory.clone();
        set_value(&mut instance_memory, 1, noun)?;
        set_value(&mut instance_memory, 2, verb)?;
        self.execute(&mut instance_memory)?;
        if instance_memory[0] == target_value {
          return Ok((100 * noun + verb).to_string());
        }
      }
    }
    Ok("No value found.".to_string())
  }
}