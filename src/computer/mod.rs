use crate::StringError;
use crate::Result;

pub struct IntcodeComputer {
  memory: Vec<i32>,
  iostream: Vec<i32>,
  pc: usize
}

impl IntcodeComputer {
  pub fn new(memory: &Vec<i32>) -> IntcodeComputer {
    IntcodeComputer { memory: memory.clone(), iostream: Vec::new(), pc: 0 }
  }

  pub fn get_value(&self, ptr: usize) -> Result<i32> {
    match self.memory.get(ptr) {
      Some(v) => Ok(*v),
      None => Err(Box::new(StringError::new("Tried to read value out of bounds.".to_string())))
    }
  }
    
  pub fn set_value(&mut self, ptr: usize, new: i32) -> Result<()> {
    if let Some(v) = self.memory.get_mut(ptr) {
      *v = new;
      Ok(())
    } else {
      return Err(Box::new(StringError::new("Tried to set out-of-bounds value.".to_string())));
    }
  }

  pub fn read(&mut self) -> i32 {
    self.iostream.remove(0)
  }

  pub fn write(&mut self, i: i32) {
    self.iostream.push(i);
  }

  fn resolve_instr(&self) -> Result<i32> {
    let instruction = self.get_value(self.pc)?;
    let instr_str = instruction.to_string();
    if instr_str.len() == 1 {
      return Ok(instruction);
    }
    Ok(instr_str[instr_str.len()-2..].parse::<i32>()?)
  }

  fn resolve_param(&self, param: i32, param_index: u8) -> Result<i32> {
    let instruction = self.get_value(self.pc)?;
    let instr_str = instruction.to_string();
    let mut instr_chars = instr_str.chars();
    for _ in 0..(param_index + 1) {
      instr_chars.next();
    }

    match instr_chars.next() {
      Some('1') => Ok(param),
      _ => Ok(self.get_value(param as usize)?)
    }
  }

  fn instr_add(&mut self) -> Result<()> {
    let a_addr = self.get_value(self.pc + 1)?;
    let b_addr = self.get_value(self.pc + 2)?;
    let a = self.resolve_param(a_addr, 0)?;
    let b = self.resolve_param(b_addr, 1)?;
    let write_addr = self.get_value(self.pc + 3)?;
    self.set_value(write_addr as usize, a + b)?;
    self.pc += 4;

    Ok(())
  }

  fn isntr_mul(&mut self) -> Result<()> {
    let a_addr = self.get_value(self.pc + 1)?;
    let b_addr = self.get_value(self.pc + 2)?;
    let a = self.resolve_param(a_addr, 0)?;
    let b = self.resolve_param(b_addr, 1)?;
    let write_addr = self.get_value(self.pc + 3)?;
    self.set_value(write_addr as usize, a * b)?;
    self.pc += 4;

    Ok(())
  }

  fn instr_input(&mut self) -> Result<()> {
    let addr = self.get_value(self.pc + 1)?;
    let val = self.read();
    self.set_value(addr as usize, val)?;
    self.pc += 2;

    Ok(())
  }

  fn instr_output(&mut self) -> Result<()> {
    let addr = self.get_value(self.pc + 1)?;
    let val = self.resolve_param(addr, 0)?;
    self.write(val);
    self.pc += 2;

    Ok(())
  }

  pub fn execute(&mut self) -> Result<()> {
    loop {
      if self.pc >= self.memory.len() {
        return Err(Box::new(StringError::new("Stack pointer pointed out of bounds.".to_string())));
      }
      let instr = self.resolve_instr()?;
      println!("{}",instr);
      match instr {
        1 => self.instr_add()?,
        2 => self.isntr_mul()?,
        3 => self.instr_input()?,
        4 => self.instr_output()?,
        99 => break,
        op => return Err(Box::new(StringError::new(format!("Invalid opcode found: {}.", op)))),
      };
    }
    Ok(())
  }
}