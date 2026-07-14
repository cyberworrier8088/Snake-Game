use macroquad::prelude::*;

pub const GRID_SIZE: i32 = 25;


pub fn grid() {

    // this for calculatibbg screen fully grid
    let cell_size = screen_width() / GRID_SIZE as f32;

    // this loop make grid
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            draw_rectangle_lines(
                x as f32 * cell_size,
                y as f32 * cell_size,
                cell_size,
                cell_size,
                1.0,
                DARKGRAY,
            );
        }
    }
}