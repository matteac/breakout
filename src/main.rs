mod entities;

use entities::{Ball, Block, Player, BLOCK_SIZE};
use macroquad::prelude::*;

pub enum GameState {
    Menu,
    Game,
    GameOver,
    Completed,
}

fn init(balls: &mut Vec<Ball>, blocks: &mut Vec<Block>, player: &mut Player, difficulty: i32) {
    balls.clear();
    blocks.clear();

    player.rect.x = screen_width() * 0.5 - player.rect.w * 0.5;

    let (w, h) = (6, 6);
    let padding = 5f32;

    let total_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_size.x * w as f32)) * 0.5f32, 40f32);

    for i in 0..w * h {
        let block_x = (i % w) as f32 * total_size.x;
        let block_y = (i / w) as f32 * total_size.y;
        let mut block = Block::new(board_start_pos + vec2(block_x, block_y));
        block.with_lives(difficulty);
        blocks.push(block);
    }
    balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));
}

#[macroquad::main("breakout")]
async fn main() {
    let debug = match std::env::var("DEBUG") {
        Ok(debug) => match debug.as_str() {
            "true" | "1" | "on" | "yes" => true,
            _ => false,
        },
        Err(_) => false,
    };
    let mut state = GameState::Menu;
    let mut difficulty = 1;
    let font = load_ttf_font("res/Anton-Regular.ttf").await.unwrap();
    let mut score = 0;

    let mut player = Player::new();
    let mut balls: Vec<Ball> = vec![];
    let mut blocks: Vec<Block> = vec![];

    init(&mut balls, &mut blocks, &mut player, difficulty);

    loop {
        clear_background(BLUE);
        for block in blocks.iter_mut() {
            block.draw();
        }
        player.draw();
        match state {
            GameState::Menu => {
                draw_main_text("Press space to start", font);
                if is_key_down(KeyCode::Space) {
                    state = GameState::Game;
                }
            }
            GameState::Game => {
                if debug {
                    if is_key_down(KeyCode::K) {
                        blocks.pop();
                    }
                    match balls.get(0) {
                        Some(ball) => {
                            draw_secondary_text(
                                format!("X:{} Y:{}", ball.velocity.x, ball.velocity.y),
                                font,
                            );
                        }
                        None => (),
                    }
                }

                if balls.len() == 0 {
                    state = GameState::GameOver;
                }
                balls.retain(|ball| ball.rect.y < screen_height());

                let score_to = balls.len() * 10;
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                    resolve_collision(&mut ball.rect, &mut ball.velocity, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.velocity, &block.rect) {
                            block.lives -= 1;
                            score += score_to;
                        }
                    }
                }
                for block in blocks.iter_mut() {
                    if block.lives == 0 && block.boost {
                        balls.push(Ball::new(block.rect.center()));
                    }
                }
                blocks.retain(|b| b.lives > 0);

                player.update(get_frame_time());

                if blocks.len() == 0 {
                    state = GameState::Completed;
                }

                draw_text_ex(
                    &format!("Score: {}", score),
                    32.0,
                    32.0,
                    TextParams {
                        font,
                        font_size: 32u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::GameOver => {
                draw_main_text("Game Over", font);
                draw_secondary_text("Press space to try again", font);

                if is_key_down(KeyCode::Space) {
                    score = 0;
                    difficulty = 1;
                    init(&mut balls, &mut blocks, &mut player, difficulty);
                    state = GameState::Game;
                }
            }
            GameState::Completed => {
                draw_main_text("You won!", font);
                draw_secondary_text("Press space to play the next level", font);
                if is_key_down(KeyCode::Space) {
                    difficulty += 1;
                    init(&mut balls, &mut blocks, &mut player, difficulty);
                    state = GameState::Game;
                }
            }
        }
        next_frame().await
    }
}

fn draw_main_text(content: impl Into<String>, font: Font) {
    let content = content.into();
    let size = 64;
    let text_dims = measure_text(&content, Some(font), size, 1.0);
    draw_text_ex(
        &content,
        (screen_width() * 0.5) - (text_dims.width * 0.5),
        (screen_height() * 0.5) + (text_dims.height * 0.5),
        TextParams {
            font,
            font_size: size,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn draw_secondary_text(content: impl Into<String>, font: Font) {
    let content = content.into();
    let size = 32;
    let text_dims = measure_text(&content, Some(font), size, 1.0);
    draw_text_ex(
        &content,
        (screen_width() * 0.5) - (text_dims.width * 0.5),
        (screen_height() * 0.5) + (text_dims.height * 2.0),
        TextParams {
            font,
            font_size: size,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
            vel.x = vel.x * rand::gen_range(0.98f32, 1.05f32);
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
            vel.y = vel.y * rand::gen_range(0.98f32, 1.05f32);
        }
    }
    true
}
