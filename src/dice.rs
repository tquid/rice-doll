use rand::{seq::SliceRandom, thread_rng, Rng};

type DieValue = i16;
type DieSize = usize;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Face {
    sign: String,
    value: DieValue,
}

impl Face {
    fn new(sign: String, value: DieValue) -> Self {
        Self { sign, value }
    }

    fn int(value: DieValue) -> Self {
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

    pub fn range(numbers: Vec<DieValue>) -> Self {
        let mut faces = Vec::<Face>::new();
        for n in numbers {
            faces.push(Face::int(n))
        }
        Self {
            faces,
            shown_face: None,
        }
    }

    pub fn int(size: DieSize) -> Self {
        let sides: Vec<DieValue> = (1..(size + 1) as DieValue).collect();
        Die::range(sides)
    }

    pub fn roll(&mut self)  -> &mut Self {
        self.shown_face = self.faces.choose(&mut rand::thread_rng()).cloned();
        self
    }

    pub fn get_shown_face(&self) -> &Face {
        self.shown_face.as_ref().unwrap()
    }

    pub fn get_size(&self) -> DieSize {
        self.faces.len()
    }

    pub fn get_faces(&self) -> &Vec<Face> {
        self.faces.as_ref()
    }
}


#[cfg(test)]
pub mod test {
    use super::{Die, Face};
    #[test]
    pub fn correct_size_from_integer() {
        let d6: Die = Die::int(6);
        assert!(Die::get_size(&d6) == 6);
    }

    #[test]
    pub fn correct_die_from_integer() -> () {
        let d6 = Die::int(6);
        assert!(d6.get_faces() == &vec![Face { sign: "1".to_owned(), value: 1 },
                                        Face { sign: "2".to_owned(), value: 2 },
                                        Face { sign: "3".to_owned(), value: 3 },
                                        Face { sign: "4".to_owned(), value: 4 },
                                        Face { sign: "5".to_owned(), value: 5 },
                                        Face { sign: "6".to_owned(), value: 6 }]);
    }
}