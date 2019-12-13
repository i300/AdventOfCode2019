use crate::Result;

pub mod one;
pub mod two;
pub mod three;

pub trait Day {
    fn run(&self) -> Result<String>;
}