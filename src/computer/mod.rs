use crate::StringError;
use crate::Result;

pub struct IntcodeComputer {
  memory: Vec<u32>
}

impl IntcodeComputer {
  pub fn new(memory: &Vec<u32>) -> IntcodeComputer {
    IntcodeComputer { memory: memory.clone() }
  }

  pub fn get_value(&self, ptr: usize) -> Result<u32> {
    match self.memory.get(ptr) {
      Some(v) => Ok(*v),
      None => Err(Box::new(StringError::new("Tried to read value out of bounds.".to_string())))
    }
  }
    
  pub fn set_value(&mut self, ptr: usize, new: u32) -> Result<()> {
    if let Some(v) = self.memory.get_mut(ptr) {
      *v = new;
      Ok(())
    } else {
      return Err(Box::new(StringError::new("Tried to set out-of-bounds value.".to_string())));
    }
  }

  fn add(&mut self, ptr: usize) -> Result<()> {
    let a_addr = self.get_value(ptr + 1)?;
    let b_addr = self.get_value(ptr + 2)?;
    let a = self.get_value(a_addr as usize)?;
    let b = self.get_value(b_addr as usize)?;
    let write_addr = self.get_value(ptr + 3)?;
    self.set_value(write_addr as usize, a + b)?;

    Ok(())
  }

  fn mul(&mut self, ptr: usize) -> Result<()> {
    let a_addr = self.get_value(ptr + 1)?;
    let b_addr = self.get_value(ptr + 2)?;
    let a = self.get_value(a_addr as usize)?;
    let b = self.get_value(b_addr as usize)?;
    let write_addr = self.get_value(ptr + 3)?;
    self.set_value(write_addr as usize, a * b)?;

    Ok(())
  }

  pub fn execute(&mut self) -> Result<()> {
    let mut ptr = 0;
    loop {
      if ptr >= self.memory.len() {
        return Err(Box::new(StringError::new("Stack pointer pointed out of bounds.".to_string())));
      }
      let code = self.memory.get(ptr);
      match code {
        Some(1) => self.add(ptr)?,
        Some(2) => self.mul(ptr)?,
        Some(99) => break,
        Some(op) => return Err(Box::new(StringError::new(format!("Invalid opcode found: {}.", op)))),
        None => return Err(Box::new(StringError::new("Accessed an out-of-bounds index".to_string()))),
      };
      ptr += 4;
    }
    Ok(())
  }
}