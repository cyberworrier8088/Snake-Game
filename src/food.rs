
use macroquad::prelude::*;
use crate::position::Position;

pub struct Food {
    pub position: Position,
}

impl Food {
    pub fn new() -> Self {
        Self {
            position: Position {
                x: 5,
                y: 5,
            },
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x as f32 * 32.0,
            self.position.y as f32 * 32.0,
            32.0,
            32.0,
            RED,
        );
    }


    pub fn respawn(&mut self) {
        self.position = Position {
            x: macroquad::rand::gen_range(0, 20),
            y: macroquad::rand::gen_range(0, 20),
        };
    }
}