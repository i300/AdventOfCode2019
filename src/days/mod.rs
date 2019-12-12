use std::error::Error;

pub mod one;

pub trait Day {
    fn new(filename: &'static str) -> Self;
    fn run(&self) -> Result<String, Box<dyn Error>>;
}