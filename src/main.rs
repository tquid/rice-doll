pub mod dice;

pub use dice::{mould_int_die as d, Pool};

fn main() {
    println!("Pool: {:?}", Pool::new(vec!(
        d(6),
        d(8),
        d(10),
        d(12),
        d(20)
    )));
}
