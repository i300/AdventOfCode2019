use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::days::Day;

pub struct One {
    filename: &'static str
}

impl One {
    fn read_file(&self) -> std::io::Result<Vec<String>> {
        let file = File::open(self.filename)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(lines)
    }
}

impl Day for One {
    fn new(filename: &'static str) -> One {
        One { filename }
    }

    fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines = self.read_file()?;

        let mut sum = 0;
        for line in lines {
            let value = line.parse::<i64>()?;
            let mut result = value / 3 - 2;
            
            let mut new_mass = result / 3 - 2;
            loop {
                result += new_mass;
                new_mass = new_mass / 3 - 2;
                if new_mass <= 0 {
                    break;
                }
            }
            
            sum += result;
        }

        Ok(sum.to_string())
    }
}