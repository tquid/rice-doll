use core::fmt::{Display, Formatter, Result};

use rand::{thread_rng, Rng};

fn rand_uint(n: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..n)
}

fn explode_roll(faces: &Vec<Face>) -> Dice {
    let max: usize = faces.len();
    let mut dice: Vec<Die> = vec!();
    loop {
        let face_index = rand_uint(max);
        let new_die: Die = Die {
            faces: faces.to_vec(),
            face_index,
        };
        dice.push(new_die);
        if face_index + 1 != max {
            break;
        }
        println!("Pop!");
    }
    Dice(dice)
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
    face_index: usize,
}

impl Die {
    fn new(faces: &Vec<Face>, roll_fn: &dyn Fn(&Vec<Face>) -> Dice) -> Dice {
        roll_fn(faces)
    }
}

#[derive(Debug)]
struct Dice (Vec<Die>);

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

impl Display for Die {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "d{}: {}", self.faces.len(), self.faces[self.face_index].1)
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output: Vec<String> = vec!();
        for die in &self.0 {
            output.push(std::format!("{}", die));
        }
        if output.len() == 1 {
            write!(f, "{}", output[0])
        } else {
            write!(f, "[{}]", output.join(", "))
        }
    }
}

impl Display for Pool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output: Vec<String> = vec!();
        for dice in &self.dice {
            output.push(std::format!("{}", dice));
        }
        write!(f, "{}", output.join(", "))
    }
}