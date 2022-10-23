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
pub enum Pool<'pool> {
    OneDie { faces: Vec<Face>, roll_fn: &dyn Fn(Vec<Face>) -> Pool },
    MultiDice { dice: Vec<Pool<'pool>> },
}

impl<'pool> Pool<'pool> {
    fn roll(&self) -> Self {
        match self {
            Pool::OneDie { faces, roll_fn } => roll_fn(faces),
            Pool::MultiDice {
                let results: Vec<Pool>,
                for (die, roll_fn) in dice {
                    results.push(roll_fn(faces))
                }
                results
            },
        }
    }
}
