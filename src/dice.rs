use core::fmt::{Display, Formatter, Result};

use rand::{seq::SliceRandom, thread_rng, Rng};

fn rand_uint(n: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..n)
}

fn roll<'die, 'face> (die: &'die Die) -> &'die Die<'die, 'face> {
    die.shown_face = die.faces.choose(&mut rand::thread_rng());
    die
}

fn explode_roll<'die, 'faces> (die: &'die Die, explode_faces: &'faces Vec<Face>) -> &'die Die<'die, 'faces> {
    let max: usize = die.faces.len();
    die.shown_face = die.faces.choose(&mut rand::thread_rng());
    loop {
        if !explode_faces.contains(die.shown_face.unwrap()) {
            break;
        }
        println!("Pop!");
        die.next_die = explode_roll(&Die::new(&die.faces, die.roll_fn), explode_faces);
    }
    &die
}

pub fn mould_int_die(num_faces: i32) -> Vec<Face> {
    let mut faces: Vec<Face> = vec!();
    for value in 1..=num_faces {
        faces.push(Face(value.to_string()));
    }
    faces
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face (pub String);

trait Roll {
    fn roll(&self) -> Self;
}

struct DieBuilder {
    faces: Vec<Face>,
    shown_face: Option<Face>,
    next_die: Option<Box<Die>>,
    roll_fn: Option<Box<dyn Fn(&Self) -> &Self>>,
}

impl DieBuilder {
    fn new(faces: Vec<Face>) -> Self {
        Self {
            faces,
            shown_face: None,
            next_die: None,
            roll_fn: None,
        }
    }
}
struct Die {
    faces: Vec<Face>,
    shown_face: Face,
    next_die: Option<Box<Die>>,
    roll_fn: Box<dyn Fn(&Self) -> &Self>,
}

impl<'die> Die<'die, 'die> {
    fn new(faces: Vec<Face>, roll_fn: Box<dyn Fn(&Die) -> &'die Die<'die, 'die>>) -> Self {
        Die {
            faces,
            shown_face: None,
            next_die: None,
            roll_fn,
        }
    }
}

pub struct Pool<'pool, 'dice> {
    dice: Vec<Die<'pool, 'dice>>,
}

impl<'pool> Pool<'pool, 'pool> {
    pub fn new(dice: Vec<Die>>) -> Self {
        let mut rolls: Vec<Die> = vec!();
        for die in dice {
            rolls.push(explode_roll(die, explode_faces))
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

impl Display for Pool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output: Vec<String> = vec!();
        for dice in &self.dice {
            output.push(std::format!("{}", dice));
        }
        write!(f, "{}", output.join(", "))
    }
}