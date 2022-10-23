use rand::{thread_rng, Rng};

fn rand_uint(n: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0..n)
}

#[derive(Debug, Clone)]
pub struct Face (pub i32, pub String);

trait Roll {
    type Die;
    fn roll(&self) -> Dice;
}

#[derive(Debug)]
struct Die {
    faces: Vec<Face>,
    face_index: u32,
}

impl Die {
    fn new(faces: &Vec<Face>) -> Self {
        Self {
            faces: faces.to_vec(),
            face_index: rand_uint(faces.len() as u32),
        }
    }
}

#[derive(Debug)]
enum Dice {
    One(Die),
    Many(Vec<Die>),
}

#[derive(Debug)]
pub struct Pool {
    dice: Vec<Dice>,
}

impl<'pool> Pool {
    pub fn new(dice: Vec<Vec<Face>>) -> Self {
        let mut rolls: Vec<Dice> = vec!();
        for die in dice {
            rolls.push(Dice::One(Die::new(&die)))
        }
        Self {
            dice: rolls,
        }
    }
}
