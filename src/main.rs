use macroquad::prelude::*;


mod grid;
mod position;
mod snake;
mod keyboard;
mod food;
mod game_over;

// this function is main funtion
#[macroquad::main("Snake Game")]
async fn main() {
    let mut snake = snake::Snake::new();
    let mut food = food::Food::new();

    let mut move_timer = 0.0;
    let mut game_over = false;

    println!("The Snake Length: {}", snake.body.len());

    loop {
        clear_background(BLACK);

        if game_over {
            game_over::draw_game_over();
            if is_key_pressed(KeyCode::R) {
                snake = snake::Snake::new();
                food = food::Food::new();
                game_over = false;
                move_timer = 0.0;
            }
            next_frame().await;
            continue;
        }
        
        if is_key_pressed(KeyCode::Up) && snake.direction != keyboard::Direction::Down {
            snake.direction = keyboard::Direction::Up;
        }

        if is_key_pressed(KeyCode::Down) && snake.direction != keyboard::Direction::Up {
            snake.direction = keyboard::Direction::Down;
        
        }

        if is_key_pressed(KeyCode::Left) && snake.direction != keyboard::Direction::Right {
            snake.direction = keyboard::Direction::Left;
        }

        if is_key_pressed(KeyCode::Right) && snake.direction != keyboard::Direction::Left {
            snake.direction = keyboard::Direction::Right;
        }

        move_timer += get_frame_time();

        if move_timer >= 0.2 {
           snake.move_snake();

           if snake.hit_self() {
                game_over = true;
           }

           if snake.eat(food.position) {
               snake.grow();
               food.respawn();
               println!("Snake Ate The Food");
           }

            move_timer = 0.0;
        }

        // Shared cell size and centering offset calculations for dynamic resizing
        let grid_pixels = screen_width().min(screen_height());
        let cell_size = grid_pixels / grid::GRID_SIZE as f32;
        let offset_x = (screen_width() - grid_pixels) / 2.0;
        let offset_y = (screen_height() - grid_pixels) / 2.0;

        grid::grid(offset_x, offset_y, cell_size);

        food.draw(offset_x, offset_y, cell_size);

        snake.draw(offset_x, offset_y, cell_size);

        next_frame().await;
    }
}