use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game_state::{GameMode, GameState};
use macroquad::audio;
use macroquad::audio::{play_sound, PlaySoundParams};
use macroquad::prelude::*;
use crate::ui::{draw_game_over, draw_game_paused, draw_top_bar, draw_win_state};

mod ball;
mod brick;
mod constants;
mod game_state;
mod level;
mod paddle;
mod ui;

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

        draw_top_bar(game_state.lives, game_state.level, game_state.total_score);

        let delta = get_frame_time();

        draw_game_paused(&game_state.mode);
        draw_game_over(&game_state.mode);
        draw_win_state(&game_state.mode);
        
        game_state.update(delta);

        next_frame().await;
    }
}
