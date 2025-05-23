use crate::constants::{BALL_RADIUS, DEFAULT_BALL_SPEED_X, DEFAULT_BALL_SPEED_Y, PADDLE_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::color::Color;
use macroquad::math::{clamp, Rect};
use macroquad::shapes::draw_circle;

pub struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
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

    pub fn update(&mut self, delta: f32, paddle_rect: &Rect) {
        let new_x = self.x + self.vel_x * delta;
        let new_y = self.y + self.vel_y * delta;

        if new_x < BALL_RADIUS || new_x > SCREEN_WIDTH - BALL_RADIUS {
            self.vel_x = -self.vel_x;
        }

        if new_y < BALL_RADIUS {
            self.vel_y = -self.vel_y;
        }

        if self.check_paddle_collision(new_x, new_y, paddle_rect) && self.vel_y > 0.0 {
            let paddle_center = paddle_rect.center();
            let hit_position = (new_x - paddle_center.x) / (paddle_rect.w / 2.0);
            let hit_position = hit_position.clamp(-1.0, 1.0);

            let current_speed = (self.vel_x * self.vel_x + self.vel_y * self.vel_y).sqrt();

            let base_angle = -std::f32::consts::PI / 2.0;
            let angle_variation = hit_position * std::f32::consts::PI / 4.0;
            let bounce_angle = base_angle + angle_variation;

            self.vel_x = bounce_angle.cos() * current_speed;
            self.vel_y = bounce_angle.sin() * current_speed;
        }

        if new_y > SCREEN_HEIGHT - BALL_RADIUS {
            println!("Game over");
            return;
        }

        self.x = clamp(self.x + self.vel_x * delta, BALL_RADIUS, SCREEN_WIDTH - BALL_RADIUS);
        self.y = clamp(self.y + self.vel_y * delta, BALL_RADIUS, SCREEN_HEIGHT - BALL_RADIUS);
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }

    fn check_paddle_collision(&self, next_x: f32, next_y: f32, paddle_rect: &Rect) -> bool {
        let ball_rect = Rect::new(next_x - self.radius, next_y - self.radius, self.radius * 2.0, self.radius * 2.0);

        paddle_rect.overlaps(&ball_rect)
    }
}
