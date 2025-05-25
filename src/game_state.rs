use macroquad::audio;
use macroquad::audio::{play_sound, stop_sound, PlaySoundParams, Sound};
use crate::ball::Ball;
use crate::brick::Brick;
use crate::constants::PADDLE_SPEED;
use crate::level::create_level_1;
use crate::paddle::Paddle;
use macroquad::color::{GOLD, MAROON};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

#[derive(PartialEq)]
pub enum GameMode {
    Playing,
    Paused,
    GameOver,
}

pub struct GameState {
    pub mode: GameMode,
    pub total_score: u32,
    pub level: u32,
    pub lives: u32,
    bg_music: Sound,
    ball_sound: Sound,
    brick_sound: Sound,
    bricks: Vec<Brick>,
    paddle: Paddle,
    ball: Ball,
}

impl GameState {
    pub fn new(bg_music: Sound, ball_sound: Sound, brick_sound: Sound) -> Self {
        let paddle = Paddle::new(GOLD);
        let ball = Ball::new(MAROON);

        let bricks = create_level_1();

        GameState {
            bricks,
            paddle,
            ball,
            bg_music,
            ball_sound,
            brick_sound,
            total_score: 0,
            level: 1,
            lives: 3,
            mode: GameMode::Paused,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if is_key_pressed(KeyCode::Space) {
            self.mode = if self.mode == GameMode::Playing {
                play_sound(
                    &self.bg_music,
                    PlaySoundParams {
                        looped: true,
                        volume: 1.0,
                    },
                );
                GameMode::Paused
            } else {
                stop_sound(&self.bg_music);
                
                GameMode::Playing
            };
        } else if self.mode == GameMode::Playing {
            if is_key_down(KeyCode::Left) {
                self.paddle.update(-PADDLE_SPEED * delta);
            }
            if is_key_down(KeyCode::Right) {
                self.paddle.update(PADDLE_SPEED * delta);
            }

            let is_ok = self.ball.update(delta, &self.paddle.rect, &self.ball_sound);
            if !is_ok {
                if self.lives == 0 {
                    self.mode = GameMode::GameOver;
                } else {
                    self.lives = self.lives.saturating_sub(1);
                    self.ball.reset();
                    self.paddle.reset_position();
                }
                
            }
        }
        self.paddle.draw();
        self.ball.draw();

        let mut hit_brick = false;

        for brick in self.bricks.iter_mut() {
            brick.draw();

            if !hit_brick && self.ball.overlaps(&brick.rect) {
                brick.lives = brick.lives.saturating_sub(1);
                hit_brick = true;
                play_sound(
                    &self.brick_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 1.0,
                    },
                );
            }
        }

        if hit_brick {
            self.ball.vel_y = -self.ball.vel_y;
            self.bricks.retain(|brick| {
                let is_alive = brick.lives > 0;
                if !is_alive {
                    self.total_score += brick.score;
                }
                is_alive
            });
        }
    }
}
