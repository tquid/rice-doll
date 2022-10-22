use rand::{thread_rng, Rng};

fn rand_int(n: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(0..n)
}

fn roll_dx(faces: Vec<Face>) -> OneDie {
    let face_index = rand_int(faces.len() as i32);
    OneDie { face_index, faces }
}

#[derive(Debug)]
struct Face {
    face_tuple: (i32, String),
}

#[derive(Debug)]
pub struct OneDie {
    face_index: i32,
    faces: Vec<Face>,
}

impl<'die> OneDie {
    pub fn new(faces: Vec<Face>, roll_fn: &'die dyn Fn(Vec<Face>) -> PoolOrDie) -> PoolOrDie {
        roll_fn(faces)
    }
}

#[derive(Debug)]
pub struct Pool<'pool> {
    dice: Vec<&'pool OneDie>,
    pools: Vec<&'pool Pool<'pool>>,
    //     placement: Area,
    //     owner: Actor,
}

enum PoolOrDie {
    Pool,
    OneDie,
}

impl Pool {
    pub fn new(rollables: Vec<(OneDie, &dyn Fn(Vec<Face>) -> PoolOrDie)>) -> Self {
        let mut dice: Vec<OneDie> = Vec::new();
        let mut pools: Vec<Pool> = Vec::new();
        for (die, roll_fn) in rollables {
            let pool_or_die = OneDie::new(die.faces, &roll_fn);
            match pool_or_die {
                PoolOrDie::OneDie => dice.push(&pool_or_die),
                PoolOrDie::Pool => pools.push(&pool_or_die),
            }
        }
        Pool { dice, pools }
    }
}
