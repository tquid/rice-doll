pub mod dice;
pub mod parser;

pub use dice::{mould_int_die as d, Pool};
pub use parser::parse;

fn main() {
    println!("{}", Pool::new(vec!(d(4), d(6), d(8), d(10), d(12), d(20))));
    parse();
}
