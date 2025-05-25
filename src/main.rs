use macroquad::prelude::*;
use crate::ball::Ball;
use crate::brick::Brick;
use crate::constants::{BRICK_WIDTH, BRICK_HEIGHT, PADDLE_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH, SPACING_WIDTH, SPACING_HEIGHT, BALL_RADIUS};
use crate::paddle::Paddle;

mod constants;
mod paddle;
mod ball;
mod brick;

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

    let mut bricks = Vec::new();
    let brick_total_width = BRICK_WIDTH + SPACING_WIDTH;
    let brick_total_height = BRICK_WIDTH + SPACING_HEIGHT;

    let num_bricks_x = ((SCREEN_WIDTH - SPACING_WIDTH) / brick_total_width) as usize;
    let num_bricks_y = 3;

    let start_y = 30.0;
    let start_x = (SCREEN_WIDTH - (num_bricks_x as f32) * brick_total_width) / 2.0;

    for i in 0..num_bricks_y {
        for j in 0..num_bricks_x {
            let x = start_x + (j as f32) * brick_total_width;
            let y = start_y + (i as f32) * brick_total_height;

            let mut brick = Brick::new();
            brick.rect = Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT);
            bricks.push(brick);
        }
    }

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

        let mut hit_brick = false;

        for brick in bricks.iter_mut() {
            brick.draw();

            if !hit_brick && ball.overlaps(&brick.rect) {
                brick.lives = brick.lives.saturating_sub(1);
                hit_brick = true;
            }
        }

        if hit_brick {
            ball.vel_y = -ball.vel_y;
            bricks.retain(|brick| brick.lives > 0);
        }

        next_frame().await;
    }
}
