pub mod dice;

pub use dice::Pool;

fn main() {
    println!("Pool: {:?}", Pool::new(6, 3))
}
