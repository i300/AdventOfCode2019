use std::error::Error;

pub mod days;

pub fn run(day_number: i32) -> Result<(), Box<dyn Error>> {
    let day = match day_number {
        1 => Ok(days::one::Day::new()),
        _ => Err("Invalid day provided")
    }?;
}