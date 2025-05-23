use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::Rect;
use macroquad::shapes::draw_rectangle;
use crate::constants::{PADDLE_HEIGHT, PADDLE_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Paddle {
    rect: Rect,
    color: Color,
}

impl Paddle {
    pub fn new(color: Color) -> Paddle {
        Paddle {
            rect: Rect::new(
                (SCREEN_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0),
                SCREEN_HEIGHT - PADDLE_HEIGHT - 10.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            color,
        }
    }

    pub fn draw(&self) {
        let rect = self.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, self.color);
    }
    
    pub fn update(&mut self, dx: f32, dy: f32) {
        self.rect.move_to(Vec2 {
            x: self.rect.x + dx,
            y: self.rect.y + dy,
        })
    }
}
