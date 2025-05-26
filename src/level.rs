use macroquad::math::Rect;
use crate::brick::{Brick, Lives};
use crate::constants::{BRICK_HEIGHT, BRICK_WIDTH, SCREEN_WIDTH, SPACING_HEIGHT, SPACING_WIDTH, TOP_BOUNDARY_HEIGHT, UI_HEIGHT};

pub fn create_level_1() -> Vec<Brick> {
    let mut bricks = Vec::new();
    
    let brick_total_width = BRICK_WIDTH + SPACING_WIDTH;
    let brick_total_height = BRICK_WIDTH + SPACING_HEIGHT;

    let num_bricks_x = 1;
    let num_bricks_y = 1;

    let start_y = UI_HEIGHT - TOP_BOUNDARY_HEIGHT + 20.0;
    let start_x = (SCREEN_WIDTH - (num_bricks_x as f32) * brick_total_width) / 2.0;

    for i in 0..num_bricks_y {
        for j in 0..num_bricks_x {
            let x = start_x + (j as f32) * brick_total_width;
            let y = start_y + (i as f32) * brick_total_height;

            let mut brick = Brick::new(Lives::One);
            brick.rect = Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT);
            bricks.push(brick);
        }
    }
    
    bricks
}

pub fn create_level_2() -> Vec<Brick> {
    let mut bricks = Vec::new();

    let brick_total_width = BRICK_WIDTH + SPACING_WIDTH;
    let brick_total_height = BRICK_WIDTH + SPACING_HEIGHT;

    let num_bricks_x = ((SCREEN_WIDTH - SPACING_WIDTH) / brick_total_width) as usize;
    let num_bricks_y = 4;

    let start_y = UI_HEIGHT - TOP_BOUNDARY_HEIGHT + 20.0;
    let start_x = (SCREEN_WIDTH - (num_bricks_x as f32) * brick_total_width) / 2.0;

    for i in 0..num_bricks_y {
        for j in 0..num_bricks_x {
            let x = start_x + (j as f32) * brick_total_width;
            let y = start_y + (i as f32) * brick_total_height;

            let lives = match i {
                0 => Lives::Three,
                1 => Lives::Two,
                _ => Lives::One,
            };

            let mut brick = Brick::new(lives);
            brick.rect = Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT);
            bricks.push(brick);
        }
    }

    bricks
}
