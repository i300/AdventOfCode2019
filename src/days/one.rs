use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;

use self::Day;

pub struct One {
}

impl Day for One {
    fn new() -> DayOne {
        DayOne {}
    }

    fn run() -> Result<String> {
        let file = File::open("input.txt")?;
        let reader = BufReader::new(file);

        let mut sum = 0;
        for line in reader.lines() {
            let value = line?.parse::<i64>().unwrap();
            let mut result = value / 3 - 2;
            
            let mut new_mass = result / 3 - 2;
            loop {
                result += new_mass;
                new_mass = new_mass / 3 - 2;
                println!("{}", new_mass);
                if new_mass <= 0 {
                    break;
                }
            }
            
            sum += result;
        }

        Ok(sum.to_string());
    }
}