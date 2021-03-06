mod days;
use days::{Day, one::One, two::Two, three::Three, four::Four, five::Five, six::Six, seven::Seven};

mod computer;

mod util;
use util::{Result, StringError};

fn get_day(num: u32) -> Option<Box<dyn Day>> {
    match num {
        1 => Some(Box::new(One::new("day1.txt"))),
        2 => Some(Box::new(Two::new("day2.txt"))),
        3 => Some(Box::new(Three::new("day3.txt"))),
        4 => Some(Box::new(Four::new("day4.txt"))),
        5 => Some(Box::new(Five::new("day5.txt"))),
        6 => Some(Box::new(Six::new("day6.txt"))),
        7 => Some(Box::new(Seven::new("day7.txt"))),
        _ => None
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few args specified.\n\tUsage: adventofcode <DAY>");
        std::process::exit(1);
    }

    let day_number = args[1].parse::<u32>()?;
    let day = match get_day(day_number) {
        Some(d) => d,
        None => return Err(Box::new(StringError::new(format!("No implementation found for day {}.", day_number))))
    };

    println!("Result: {}", day.run()?);

    Ok(())
}