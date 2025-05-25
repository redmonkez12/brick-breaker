use crate::constants::{
    PADDLE_HEIGHT, PADDLE_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, UI_HEIGHT,
};
use macroquad::color::Color;
use macroquad::math::{clamp, Vec2};
use macroquad::prelude::Rect;
use macroquad::shapes::draw_rectangle;

pub struct Paddle {
    pub rect: Rect,
    color: Color,
}

impl Paddle {
    pub fn new(color: Color) -> Self {
        Paddle {
            rect: Paddle::get_default_rect(),
            color,
        }
    }

    pub fn draw(&self) {
        let rect = self.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, self.color);
    }

    pub fn update(&mut self, dx: f32) {
        let clamped_x = clamp(self.rect.x + dx, UI_HEIGHT, SCREEN_WIDTH - PADDLE_WIDTH);
        self.rect.move_to(Vec2::new(clamped_x, self.rect.y));
    }
    
    pub fn reset_position(&mut self) {
        self.rect = Paddle::get_default_rect();
    }

    fn get_default_rect() -> Rect {
        Rect::new(
            (SCREEN_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0),
            SCREEN_HEIGHT - PADDLE_HEIGHT - 10.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )
    }
}
