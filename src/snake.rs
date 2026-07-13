use macroquad::prelude::*;
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
    
    // i am drawing a snake here
    pub fn draw(&self) {
        for part in &self.body {
            draw_rectangle(
                part.x as f32 * 32.0,
                part.y as f32 * 32.0,
                32.0,
                32.0,
                GREEN,
            );
        }
    }
}