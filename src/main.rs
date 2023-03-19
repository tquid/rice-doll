pub mod dice;
pub mod parser;

pub use dice::{Die};
pub use parser::parse;

fn main() {
    // println!("{}", Pool::new(vec!(d(4), d(6), d(8), d(10), d(12), d(20))));
    // parse();
    let mut d6 = Die::int(6);
    let mut d6 = Die::int(6);
    d6.roll();
    println!("Face showing: {:?}", d6.get_shown_face());
    println!("This is a {}-sided die", d6.get_size());
    println!("Faces: {:?}", d6.get_faces());
    let d_null = Die::int(0);
    println!("Null die: {:?}", d_null);
}
