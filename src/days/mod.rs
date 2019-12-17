use crate::Result;

pub mod one;
pub mod two;
pub mod three;
pub mod four;

pub trait Day {
    fn run(&self) -> Result<String>;
}