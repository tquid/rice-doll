use core::num;

use rand::{thread_rng, Rng};

fn rand_uint(n: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0..n)
}

pub fn explode_roll(faces: &Vec<Face>) -> Dice {
    let max: u32 = faces.len() as u32;
    let mut dice: Vec<Die> = vec!();
    loop {
        let face_index = rand_uint(max);
        let new_die: Die = Die {
            faces: faces.to_vec(),
            face_index,
        };
        dice.push(new_die);
        if face_index != max {
            break;
        }
    }
    if dice.len() == 1 {
        Dice::One(&dice[0])
    } else {
        Dice::Many(dice)
    }
}

pub fn mould_int_die(num_faces: i32) -> Vec<Face> {
    let mut faces: Vec<Face> = vec!();
    for value in 1..=num_faces {
        faces.push(Face(value, value.to_string()));
    }
    faces
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
    fn new(faces: &Vec<Face>, roll_fn: &dyn Fn(&Vec<Face>) -> Dice) -> Dice {
        roll_fn(faces)
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

impl Pool {
    pub fn new(dice: Vec<Vec<Face>>) -> Self {
        let mut rolls: Vec<Dice> = vec!();
        for die in dice {
            rolls.push(Die::new(&die, &explode_roll))
        }
        Self {
            dice: rolls,
        }
    }
}
