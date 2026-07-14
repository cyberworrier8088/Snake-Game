use macroquad::prelude::*;

pub const GRID_SIZE: i32 = 25;


pub fn grid(offset_x: f32, offset_y: f32, cell_size: f32) {
    // this loop make grid
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            draw_rectangle_lines(
                offset_x + x as f32 * cell_size,
                offset_y + y as f32 * cell_size,
                cell_size,
                cell_size,
                1.0,
                DARKGRAY,
            );
        }
    }
}