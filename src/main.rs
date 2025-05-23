use macroquad::prelude::*;
use crate::ball::Ball;
use crate::constants::{PADDLE_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::paddle::Paddle;

mod constants;
mod paddle;
mod ball;

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
    let mut paddle = Paddle::new(GOLD);
    let mut ball = Ball::new(MAROON);

    loop {
        clear_background(BLACK);

        let delta = get_frame_time();

        if is_key_down(KeyCode::Left) {
            paddle.update(-PADDLE_SPEED * delta);
        }
        if is_key_down(KeyCode::Right) {
            paddle.update(PADDLE_SPEED * delta);
        }

        paddle.draw();
        ball.update(delta, &paddle.rect);
        ball.draw();

        next_frame().await;
    }
}
