use std::error::Error;
use std::fmt;

mod days;
use crate::days::Day;
use crate::days::one::One;
use crate::days::two::Two;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_day(num: u32) -> Option<Box<dyn Day>> {
    match num {
        1 => Some(Box::new(One::new("day1.txt"))),
        2 => Some(Box::new(Two::new("day2.txt"))),
        _ => None
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few args specified.\n\tUsage: adventofcode <DAY>")
    }

    let day_number = args[1].parse::<u32>()?;
    let day = match get_day(day_number) {
        Some(d) => d,
        None => return Err(Box::new(StringError(format!("No implementation found for day {}.", day_number))))
    };

    println!("Result: {}", day.run()?);

    Ok(())
}

#[derive(Debug)]
pub struct StringError(String);

impl fmt::Display for StringError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for StringError {}