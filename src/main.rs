use macroquad::prelude::*;


#[macroquad::main("Snake Game")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Snake Game", 200.0, 200.0, 50.0, MAGENTA);

        next_frame().await;
    }
}