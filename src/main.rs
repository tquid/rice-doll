pub mod dice;
pub mod parser;

pub use dice::{Die};
pub use parser::parse;

fn main() {
    // println!("{}", Pool::new(vec!(d(4), d(6), d(8), d(10), d(12), d(20))));
    // parse();
    let mut d6 = Die::from(6);
    d6.roll();
    println!("Face showing: {:?}", d6.get_face());
}
