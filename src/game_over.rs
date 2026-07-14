use macroquad::prelude::*;

pub fn draw_game_over() {
    draw_text(
        "GAME OVER",
        140.0,
        250.0,
        60.0,
        RED,
    );

    draw_text(
        "Press R to Restart",
        100.0,
        320.0,
        35.0,
        WHITE,
    );
}