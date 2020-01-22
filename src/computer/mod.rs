use crate::StringError;
use crate::Result;

pub struct IntcodeComputer<'a> {
  memory: &'a mut Vec<i32>,
  iostream: Vec<i32>,
  pc: usize
}

impl IntcodeComputer<'_> {
  pub fn new(memory: &mut Vec<i32>) -> IntcodeComputer {
    IntcodeComputer { memory: memory, iostream: Vec::new(), pc: 0 }
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

  pub fn read(&mut self) -> Option<i32> {
    self.iostream.pop()
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

  fn resolve_param(&self, param: i32, param_index: usize) -> Result<i32> {
    let instruction = self.get_value(self.pc)?;
    let instr_str = instruction.to_string();
    let mut instr_chars = instr_str.chars().rev();

    match instr_chars.nth(param_index + 2) {
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
    let param = self.get_value(self.pc + 1)?;
    if let Some(val) = self.read() {
      self.set_value(param as usize, val)?;
      self.pc += 2;
      Ok(())
    } else {
      Err(Box::new(StringError::new("Tried to read from empty iostream".to_string())))
    }
  }

  fn instr_output(&mut self) -> Result<()> {
    let param = self.get_value(self.pc + 1)?;
    let val = self.resolve_param(param, 0)?;
    self.write(val);
    self.pc += 2;

    Ok(())
  }

  fn instr_jnz(&mut self) -> Result<()> {
    let cond_addr = self.get_value(self.pc + 1)?;
    let loc_addr = self.get_value(self.pc + 2)?;
    let cond = self.resolve_param(cond_addr, 0)?;
    let addr = self.resolve_param(loc_addr, 1)?;
    if cond != 0 {
      self.pc = addr as usize;
    } else {
      self.pc += 3;
    }

    Ok(())
  }

  fn instr_jz(&mut self) -> Result<()> {
    let cond_addr = self.get_value(self.pc + 1)?;
    let loc_addr = self.get_value(self.pc + 2)?;
    let cond = self.resolve_param(cond_addr, 0)?;
    let addr = self.resolve_param(loc_addr, 1)?;
    if cond == 0 {
      self.pc = addr as usize;
    } else {
      self.pc += 3;
    }

    Ok(())
  }

  fn instr_lt(&mut self) -> Result<()> {
    let a_addr = self.get_value(self.pc + 1)?;
    let b_addr = self.get_value(self.pc + 2)?;
    let target_addr = self.get_value(self.pc + 3)?;
    let a = self.resolve_param(a_addr, 0)?;
    let b = self.resolve_param(b_addr, 1)?;
    self.set_value(target_addr as usize, if a < b { 1 } else { 0 })?;
    self.pc += 4;

    Ok(())
  }

  fn instr_eq(&mut self) -> Result<()> {
    let a_addr = self.get_value(self.pc + 1)?;
    let b_addr = self.get_value(self.pc + 2)?;
    let target_addr = self.get_value(self.pc + 3)?;
    let a = self.resolve_param(a_addr, 0)?;
    let b = self.resolve_param(b_addr, 1)?;
    self.set_value(target_addr as usize, if a == b { 1 } else { 0 })?;
    self.pc += 4;

    Ok(())
  }

  pub fn execute(&mut self) -> Result<()> {
    loop {
      if self.pc >= self.memory.len() {
        return Err(Box::new(StringError::new("Stack pointer pointed out of bounds.".to_string())));
      }
      let instr = self.resolve_instr()?;
      match instr {
        1 => self.instr_add()?,
        2 => self.isntr_mul()?,
        3 => self.instr_input()?,
        4 => self.instr_output()?,
        5 => self.instr_jnz()?,
        6 => self.instr_jz()?,
        7 => self.instr_lt()?,
        8 => self.instr_eq()?,
        99 => break,
        op => return Err(Box::new(StringError::new(format!("Invalid opcode found: {}.", op)))),
      };
    }
    Ok(())
  }
}