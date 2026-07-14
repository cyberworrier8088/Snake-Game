use macroquad::prelude::*;


mod grid;
mod position;
mod snake;
mod keyboard;
mod food;

// this function is main funtion
#[macroquad::main("Snake Game")]
async fn main() {
    let mut snake = snake::Snake::new();
    let mut food = food::Food::new();

    let mut move_timer = 0.0;

    println!("The Snake Length: {}", snake.body.len());

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up) {
            snake.direction = keyboard::Direction::Up;
        }

        if is_key_pressed(KeyCode::Down) {
            snake.direction = keyboard::Direction::Down;
        
        }

        if is_key_pressed(KeyCode::Left) {
            snake.direction = keyboard::Direction::Left;
        
        }

        if is_key_pressed(KeyCode::Right) {
            snake.direction = keyboard::Direction::Right;
        
        }

        move_timer += get_frame_time();

        if move_timer >= 0.2 {
           snake.move_snake();

           if snake.eat(food.position) {
               snake.grow();
               food.respawn();
               println!("Snake Ate The Food");
           }

            move_timer = 0.0;
        }

        grid::grid();

        food.draw();

        snake.draw();

        next_frame().await;
    }
}