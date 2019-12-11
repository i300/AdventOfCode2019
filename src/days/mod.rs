use std::error::Error;

pub mod one;

pub trait Day {
    fn new() -> Self;
    fn run() -> Result<String, Box<dyn Error>>;
}