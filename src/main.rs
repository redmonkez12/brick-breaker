use macroquad::prelude::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::paddle::Paddle;

mod constants;
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
    let paddle = Paddle::new(GOLD);
    
    loop {
        clear_background(BLACK);

        paddle.draw();
        
        next_frame().await;
    }
}
