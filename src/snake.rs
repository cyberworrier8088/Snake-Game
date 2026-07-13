use macroquad::prelude::*;
use crate::position::Position;
use crate::keyboard::Direction;

pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
}


impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Position { x: 10, y: 10 },
                Position { x: 10, y: 11 },
                Position { x: 10, y: 12 },
            ],
            direction: Direction::Up,
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

    pub fn move_up(&mut self) {
        let head = self.body[0];

        let new_head = Position {
            x: head.x,
            y: head.y - 1,
        };

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_down(&mut self) {
        let head = self.body[0];

        let new_head = Position {
            x: head.x,
            y: head.y + 1,
        };

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_left(&mut self) {
        let head = self.body[0];

        let new_head = Position {
            x: head.x -1,
            y: head.y,
        };

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_right(&mut self) {
        let head = self.body[0];

        let new_head = Position {
            x: head.x + 1,
            y: head.y,
        };

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_snake(&mut self) {
        match self.direction {
            Direction::Up => {
                self.move_up();
            }

            Direction::Down => {
                self.move_down();
            }

            Direction::Left => {
                self.move_left();
            }

            Direction::Right => {
                self.move_right();
            }
        }
    }
}