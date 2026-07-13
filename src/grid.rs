use macroquad::prelude::*;
pub const GRID_SIZE: i32 = 20;
pub const CELL_SIZE: f32 = 32.0;


pub fn grid() {
    // this loop make grid
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            draw_rectangle_lines(
                x as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                1.0,
                DARKGRAY,
            );
        }
    }
}