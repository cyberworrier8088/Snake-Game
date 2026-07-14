
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

    pub fn draw(&self, offset_x: f32, offset_y: f32, cell_size: f32) {
        draw_rectangle(
            offset_x + self.position.x as f32 * cell_size,
            offset_y + self.position.y as f32 * cell_size,
            cell_size,
            cell_size,
            RED,
        );
    }


    pub fn respawn(&mut self) {
        self.position = Position {
            x: macroquad::rand::gen_range(0, crate::grid::GRID_SIZE),
            y: macroquad::rand::gen_range(0, crate::grid::GRID_SIZE),
        };
    }
}