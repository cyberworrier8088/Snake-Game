use macroquad::prelude::*;


mod grid;

// this function is main funtion
#[macroquad::main("Snake Game")]
async fn main() {
    loop {
        clear_background(BLACK);

        grid::grid();

        next_frame().await;
    }
}