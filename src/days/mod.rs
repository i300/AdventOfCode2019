pub mod one;
pub mod two;

use crate::Result;

pub trait Day {
    fn run(&self) -> Result<String>;
}