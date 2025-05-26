use macroquad::color::WHITE;
use macroquad::prelude::{draw_line, draw_text, measure_text};
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, UI_HEIGHT};
use crate::game_state::GameMode;

pub fn draw_top_bar(lives: u32, level: u32, total_score: u32) {
    let lives_text = format!("Lives: {}", lives);
    let level_text = format!("Level: {}", level);
    let level_size = measure_text(&level_text, None, 20, 1.0);
    let lives_size = measure_text(&lives_text, None, 20, 1.0);

    draw_line(0.0, UI_HEIGHT, SCREEN_WIDTH, UI_HEIGHT, 1.0, WHITE);
    draw_text(&lives_text, 20.0, UI_HEIGHT - 15.0, 20.0, WHITE);
    draw_text(&level_text, 20.0 + lives_size.width + 10.0, UI_HEIGHT - 15.0, 20.0, WHITE);

    draw_text(
        &format!("Score: {}", total_score),
        20.0 + level_size.width + lives_size.width + 20.0,
        UI_HEIGHT - 15.0,
        20.0,
        WHITE,
    );
}

pub fn draw_game_paused(mode: &GameMode) {
    if *mode == GameMode::Paused {
        let text = "Press Space to start";
        let text_size = measure_text(text, None, 20, 1.0);

        draw_text(
            text,
            SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - text_size.height / 2.0,
            20.0,
            WHITE,
        );
    }
}

pub fn draw_game_over(mode: &GameMode) {
    if *mode == GameMode::GameOver {
        let text = "Game Over press Space to restart";
        let text_size = measure_text(text, None, 20, 1.0);

        draw_text(
            text,
            SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - text_size.height / 2.0,
            20.0,
            WHITE,
        );
    }
}

pub fn draw_win_state(mode: &GameMode) {
    if *mode == GameMode::Win {
        let text = "You won! press Space to restart";
        let text_size = measure_text(text, None, 20, 1.0);

        draw_text(
            text,
            SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - text_size.height / 2.0,
            20.0,
            WHITE,
        );
    }
}
