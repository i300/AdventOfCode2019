use crate::days::Day;
use crate::StringError;
use crate::Result;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/* -- Main Type -- */
pub struct Two {
  filename: &'static str
}

fn get_value(stack: &Vec<u32>, ptr: usize) -> Result<u32> {
  match stack.get(ptr) {
    Some(v) => Ok(*v),
    None => Err(Box::new(StringError("Tried to read value out of bounds.".to_string())))
  }
}

fn set_value(stack: &mut Vec<u32>, ptr: usize, new: u32) -> Result<()> {
  if let Some(v) = stack.get_mut(ptr) {
    *v = new;
    Ok(())
  } else {
    return Err(Box::new(StringError("Tried to set out-of-bounds value.".to_string())));
  }
}

impl Two {
  pub fn new(filename: &'static str) -> Two {
    Two { filename }
  }

  fn read_file(&self) -> std::io::Result<String> {
    let file = File::open(self.filename)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
  }

  fn add(&self, stack: &mut Vec<u32>, ptr: usize) -> Result<()> {
    let a_addr = get_value(stack, ptr + 1)?;
    let b_addr = get_value(stack, ptr + 2)?;
    let a = get_value(stack, a_addr as usize)?;
    let b = get_value(stack, b_addr as usize)?;
    let write_addr = get_value(stack, ptr + 3)?;
    set_value(stack, write_addr as usize, a + b)?;

    Ok(())
  }

  fn mul(&self, stack: &mut Vec<u32>, ptr: usize) -> Result<()> {
    let a_addr = get_value(stack, ptr + 1)?;
    let b_addr = get_value(stack, ptr + 2)?;
    let a = get_value(stack, a_addr as usize)?;
    let b = get_value(stack, b_addr as usize)?;
    let write_addr = get_value(stack, ptr + 3)?;
    set_value(stack, write_addr as usize, a * b)?;

    Ok(())
  }
}

impl Day for Two {
  fn run(&self) -> Result<String> {
    let contents = self.read_file()?;
    let mut stack: Vec<u32> = match contents.split(",").map(|s| s.parse::<u32>()).collect() {
      Ok(s) => s,
      Err(e) => return Err(Box::new(e))
    };
    let mut ptr = 0;
    loop {
      if ptr >= stack.len() {
        return Err(Box::new(StringError("Stack pointer pointed out of bounds.".to_string())));
      }
      let code = stack.get(ptr);
      match code {
        Some(1) => self.add(&mut stack, ptr)?,
        Some(2) => self.mul(&mut stack, ptr)?,
        Some(99) => break,
        Some(op) => return Err(Box::new(StringError(format!("Invalid opcode found: {}.", op)))),
        None => return Err(Box::new(StringError("Accessed an out-of-bounds index".to_string()))),
      };
      ptr += 4;
    }
    Ok(stack[0].to_string())
  }
}