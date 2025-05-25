use macroquad::color::{Color, LIME, ORANGE, RED};
use macroquad::math::Rect;
use macroquad::prelude::draw_rectangle;

pub enum Lives {
    One = 1,
    Two = 2,
    Three = 3,
}

pub struct Brick {
    pub lives: u8,
    pub rect: Rect,
    pub score: u32,
    colors: Vec<Color>,
}

impl Brick {
    pub fn new(lives: Lives) -> Self {
        Self {
            lives: lives as u8,
            rect: Rect::default(),
            colors: vec![RED, ORANGE, LIME],
            score: 100,
        }
    }

    pub fn draw(&self) {
        let rect = &self.rect;
        let color = self.colors[self.lives as usize - 1];

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    }
}
