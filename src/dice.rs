use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct OneDie {
    value: i64,
    faces: Vec<i64>,
}

impl OneDie {
    // pub fn roll(mut die: OneDie<'die>) -> Self {
    //     let mut rng = thread_rng();
    //     die.value = die.faces.choose(&mut rng).as_mut();
    //     die
    // }

    pub fn new(faces: Vec<i64>) -> Self {
        let mut rng = thread_rng();
        let value = *faces.choose(&mut rng).unwrap();
        OneDie { faces, value }
    }
}

#[derive(Debug)]
pub struct Pool {
    dice: Vec<OneDie>,
    //     placement: Area,
    //     owner: Actor,
}

impl Pool {
    pub fn new(faces: i64, number: i64) -> Self {
        let mut dice: Vec<OneDie> = Vec::new();
        for _ in 1..=number {
            let die = OneDie::new((1..=faces).collect());
            dice.push(die);
        }
        Pool { dice }
    }
}
