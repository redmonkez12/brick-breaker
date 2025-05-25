use macroquad::audio::{PlaySoundParams, Sound};
use crate::constants::{BALL_RADIUS, DEFAULT_BALL_SPEED_X, DEFAULT_BALL_SPEED_Y, PADDLE_HEIGHT, PLAYGROUND_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, UI_HEIGHT};
use macroquad::color::Color;
use macroquad::math::{clamp, Rect};
use macroquad::shapes::draw_circle;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    radius: f32,
    color: Color,
}

impl Ball {
    pub fn new(color: Color) -> Self {
        Self {
            x: SCREEN_WIDTH / 2.0,
            y: SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_RADIUS * 2.0,
            radius: BALL_RADIUS,
            vel_x: DEFAULT_BALL_SPEED_X,
            vel_y: -DEFAULT_BALL_SPEED_Y,
            color,
        }
    }

    pub fn update(&mut self, delta: f32, paddle_rect: &Rect, ball_sound: &Sound) -> bool {
        let new_x = self.x + self.vel_x * delta;
        let new_y = self.y + self.vel_y * delta;
        let mut play_sound = false;

        if new_x < BALL_RADIUS || new_x > SCREEN_WIDTH - BALL_RADIUS {
            self.vel_x = -self.vel_x;
            play_sound = true;
        }

        if new_y < BALL_RADIUS + UI_HEIGHT {
            self.vel_y = -self.vel_y;
            play_sound = true;
        }
        
        if self.overlaps(paddle_rect) && self.vel_y > 0.0 {
            let paddle_center = paddle_rect.center();
            let hit_position = (new_x - paddle_center.x) / (paddle_rect.w / 2.0);
            let hit_position = hit_position.clamp(-1.0, 1.0);

            let current_speed = (self.vel_x * self.vel_x + self.vel_y * self.vel_y).sqrt();

            let base_angle = -std::f32::consts::PI / 2.0;
            let angle_variation = hit_position * std::f32::consts::PI / 4.0;
            let bounce_angle = base_angle + angle_variation;

            self.vel_x = bounce_angle.cos() * current_speed;
            self.vel_y = bounce_angle.sin() * current_speed;
            play_sound = true;
        }

        if play_sound {
            macroquad::audio::play_sound(
                ball_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
        }

        if new_y > PLAYGROUND_HEIGHT - BALL_RADIUS {
            return false;
        }

        self.x = clamp(
            self.x + self.vel_x * delta,
            BALL_RADIUS,
            SCREEN_WIDTH - BALL_RADIUS,
        );
        self.y = clamp(
            self.y + self.vel_y * delta,
            BALL_RADIUS,
            PLAYGROUND_HEIGHT - BALL_RADIUS,
        );
        
        true
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let closest_x = self.x.clamp(other.x, other.x + other.w);
        let closest_y = self.y.clamp(other.y, other.y + other.h);

        let distance_x = self.x - closest_x;
        let distance_y = self.y - closest_y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;

        distance_squared <= (self.radius * self.radius)
    }
    
    pub fn reset(&mut self) {
        self.x = SCREEN_WIDTH / 2.0;
        self.y = SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_RADIUS * 2.0;
        self.vel_x = DEFAULT_BALL_SPEED_X;
        self.vel_y = -DEFAULT_BALL_SPEED_Y;
    }
}
