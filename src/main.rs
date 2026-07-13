use macroquad::prelude::*;


mod grid;
mod position;
mod snake;

// this function is main funtion
#[macroquad::main("Snake Game")]
async fn main() {
    let snake = snake::Snake::new();

    println!("The Snake Length: {}", snake.body.len());

    loop {
        clear_background(BLACK);

        grid::grid();

        snake.draw();

        next_frame().await;
    }
}