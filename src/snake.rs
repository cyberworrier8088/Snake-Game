use crate::position::Position;

pub struct Snake {
    pub body: Vec<Position>,
}


impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Position { x: 10, y: 10 },
                Position { x: 10, y: 11 },
                Position { x: 10, y: 12 },
            ],
        }
    }
}