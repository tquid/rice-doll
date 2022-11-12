extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use dice::{Dice, Pool, Roll};
pub use parser::parse_command;

pub mod dice;
