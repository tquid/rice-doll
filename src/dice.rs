use rand::{seq::SliceRandom, thread_rng, Rng};

type DieValue = i16;
type DieSize = usize;

#[derive(Debug, Clone)]
pub struct Face {
    sign: String,
    value: DieValue,
}

impl Face {
    fn new(sign: String, value: DieValue) -> Self {
        Self { sign, value }
    }
}

impl From<DieValue> for Face {
    fn from(value: DieValue) -> Self {
        Face::new(value.to_string(), value)
    }
}

#[derive(Debug)]
pub struct Die {
    faces: Vec<Face>,
    shown_face: Option<Face>,
}

// Die::new(Vec<Face>) to create a fully custom die with arbitrary signs and
// values
impl Die {
    pub fn new(faces: Vec<Face>) -> Self {
        Self {
            faces,
            shown_face: None,
        }
    }

    pub fn roll(&mut self)  -> &mut Self {
        self.shown_face = self.faces.choose(&mut rand::thread_rng()).cloned();
        self
    }

    pub fn get_face(self) -> Face {
        self.shown_face.unwrap()
    }
}

// Die::from(Vec<DieValue>) to create a die with arbitrary integer faces
impl From<Vec<DieValue>> for Die {
    fn from(numbers: Vec<DieValue>) -> Self {
        let mut faces = Vec::<Face>::new();
        for n in numbers {
            faces.push(Face::from(n))
        }
        Self {
            faces,
            shown_face: None,
        }
    }
}

// Use Die::from(DieSize) to create a typical die numbered 1..n
impl From<DieSize> for Die {
    fn from(size: DieSize) -> Self {
        let sides: Vec<DieValue> = (1..size as DieValue).collect();
        Die::from(sides)
    }
}
