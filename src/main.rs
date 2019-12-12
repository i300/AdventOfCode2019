mod days;
use crate::days::Day;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few args specified.\n\tUsage: adventofcode <DAY>")
    }

    let day_number = args[1].parse::<i32>()?;
    let day = match day_number {
        1 => Ok(days::one::One::new("day1.txt")),
        _ => Err("Invalid day provided")
    }?;

    println!("Result: {}", day.run()?);

    Ok(())
}
