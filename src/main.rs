pub mod dice;

pub use dice::{Face, Pool};

fn main() {
    println!("Pool: {:?}", Pool::new(vec!(vec!(
        Face(1, "1".to_string()),
        Face(2, "2".to_string()),
        Face(3, "3".to_string()),
        Face(4, "4".to_string()),
        Face(5, "5".to_string()),
        Face(6, "6".to_string()),
    ))));
}
