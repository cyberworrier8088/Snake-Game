use macroquad::prelude::*;


mod grid;
mod position;
mod snake;
mod keyboard;

// this function is main funtion
#[macroquad::main("Snake Game")]
async fn main() {
    let mut snake = snake::Snake::new();

    let mut move_timer = 0.0;

    println!("The Snake Length: {}", snake.body.len());

    loop {
        clear_background(BLACK);

        move_timer += get_frame_time();

        if move_timer >= 0.2 {
            snake.move_up();
            move_timer = 0.0;
        }

        grid::grid();

        snake.draw();

        next_frame().await;
    }
}