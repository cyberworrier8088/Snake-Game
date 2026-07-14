use macroquad::prelude::*;

pub fn draw_game_over() {
    let title_text = "GAME OVER";
    let sub_text = "Press R to Restart";

    let title_font_size = 60.0;
    let sub_font_size = 35.0;

    let title_dims = measure_text(title_text, None, title_font_size as u16, 1.0);
    let sub_dims = measure_text(sub_text, None, sub_font_size as u16, 1.0);

    let screen_w = screen_width();
    let screen_h = screen_height();

    // Center title vertically and horizontally
    let title_x = (screen_w - title_dims.width) / 2.0;
    let title_y = (screen_h - title_dims.height) / 2.0;

    // Subtitle goes below title
    let sub_x = (screen_w - sub_dims.width) / 2.0;
    let sub_y = title_y + title_dims.height + 20.0;

    draw_text(
        title_text,
        title_x,
        title_y,
        title_font_size,
        RED,
    );

    draw_text(
        sub_text,
        sub_x,
        sub_y,
        sub_font_size,
        WHITE,
    );
}