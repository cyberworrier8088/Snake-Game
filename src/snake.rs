use macroquad::prelude::*;
use crate::position::Position;
use crate::keyboard::Direction;
use crate::grid::GRID_SIZE;

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

        let cell_size = screen_width() / GRID_SIZE as f32;

        for part in &self.body {
            draw_rectangle(
                part.x as f32 * cell_size,
                part.y as f32 * cell_size,
                cell_size,
                cell_size,
                GREEN,
            );
        }
    }

    pub fn move_up(&mut self) {
        let head = self.body[0];

        let mut new_head = Position {
            x: head.x,
            y: head.y - 1,
        };

        if new_head.y < 0 {
            println!("WRAP UP");
            new_head.y = GRID_SIZE - 1;
        }

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_down(&mut self) {
        let head = self.body[0];

        let mut new_head = Position {
            x: head.x,
            y: head.y + 1,
        };

        if new_head.y >= GRID_SIZE{

            println!("WRAP DOWN");
            new_head.y = 0;
        }

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_left(&mut self) {
        let head = self.body[0];

        let mut new_head = Position {
            x: head.x -1,
            y: head.y,
        };

        if new_head.x < 0 {
            println!("WRAP LEFT");
            new_head.x = GRID_SIZE - 1;
        }

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn move_right(&mut self) {
        let head = self.body[0];

        let mut new_head = Position {
            x: head.x + 1,
            y: head.y,
        };

        if new_head.x >= GRID_SIZE {
            println!("WRAP RIGHT");
            new_head.x = 0;
        }

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


    pub fn eat(&self, food_position: Position) -> bool {
        self.body[0] == food_position
    }

    pub fn grow(&mut self) {
        let tail = self.body[self.body.len() - 1];

        self.body.push(tail);
    }

    pub fn hit_self(&self) -> bool {
        println!("Head: {:?}", self.body[0]);
        let head = self.body[0];

        for part in &self.body[1..] {
            if *part == head {
                return true;
            }
        }

        false
    }
}