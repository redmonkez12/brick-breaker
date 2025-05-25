use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, UI_HEIGHT};
use crate::game_state::{GameMode, GameState};
use macroquad::audio;
use macroquad::audio::{play_sound, PlaySoundParams};
use macroquad::prelude::*;

mod ball;
mod brick;
mod constants;
mod game_state;
mod level;
mod paddle;

fn window_conf() -> Conf {
    Conf {
        window_title: "Brick breaker".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("src/assets");
    let bg_music = audio::load_sound("sounds/game_music.wav").await.unwrap();
    let ball_sound = audio::load_sound("sounds/ball_sound.wav").await.unwrap();
    let brick_sound = audio::load_sound("sounds/brick_sound.wav").await.unwrap();

    play_sound(
        &bg_music,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );
    
    let mut game_state = GameState::new(bg_music, ball_sound, brick_sound);

    loop {
        clear_background(BLACK);

        let lives_text = format!("Lives: {}", game_state.lives);
        let level_text = format!("Level: {}", game_state.level);
        let level_size = measure_text(&level_text, None, 20, 1.0);
        let lives_size = measure_text(&lives_text, None, 20, 1.0);

        draw_line(0.0, UI_HEIGHT, SCREEN_WIDTH, UI_HEIGHT, 1.0, WHITE);
        draw_text(&lives_text, 20.0, UI_HEIGHT - 15.0, 20.0, WHITE);
        draw_text(&level_text, 20.0 + lives_size.width + 10.0, UI_HEIGHT - 15.0, 20.0, WHITE);

        draw_text(
            &format!("Score: {}", game_state.total_score),
            20.0 + level_size.width + lives_size.width + 20.0,
            UI_HEIGHT - 15.0,
            20.0,
            WHITE,
        );

        let delta = get_frame_time();

        if game_state.mode == GameMode::Paused {
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
        
        if game_state.mode == GameMode::GameOver {
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

        game_state.update(delta);

        next_frame().await;
    }
}
