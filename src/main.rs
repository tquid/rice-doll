pub mod dice;

pub use dice::{OneDie, Pool};

fn main() {
    println!("Pool: {:?}", Pool::new(6, 3))
}
