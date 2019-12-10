extern crate adventofcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few args specified.\n\tUsage: adventofcode <DAY>")
    }

    let day = args[1].parse::<i32>()?;
    
    adventofcode::run(day);

    Ok(())
}
