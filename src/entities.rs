use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 700f32;

pub const BLOCK_SIZE: Vec2 = Vec2::from_array([120f32, 30f32]);

pub fn player_size() -> Vec2 {
    return Vec2::from_array([screen_width() / 5f32, screen_width() / 20f32]);
}
pub fn ball_size() -> Vec2 {
    return Vec2::from_array([screen_width() / 50f32, screen_width() / 50f32]);
}
const BALL_SPEED: f32 = 300f32;

pub struct Player {
    pub rect: Rect,
    pub lives: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect {
                x: screen_width() * 0.5f32 - (player_size().x * 2f32),
                y: screen_height() - player_size().y * 2.0,
                w: player_size().x,
                h: player_size().y,
            },
            lives: 3,
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.rect.w = screen_width() / 5.0;
        self.rect.h = screen_height() / 20.0;
        self.rect.y = screen_height() - self.rect.h * 2.0;

        let x_move = match (
            (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)),
            (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)),
        ) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }

        self.draw();
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub rect: Rect,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        let mut vel = vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize();
        if vel.x > 0.0 {
            if vel.x < 0.2 {
                vel.x = 0.2
            }
        }
        if vel.x < 0.0 {
            if vel.x > -0.2 {
                vel.x = -0.2
            }
        }
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: ball_size().x,
                h: ball_size().y,
            },
            velocity: vel,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let speed = match is_key_down(KeyCode::L) {
            false => BALL_SPEED,
            true => {
                if std::env::var("DEBUG").is_ok() {
                    match std::env::var("DEBUG").unwrap().as_str() {
                        "true" | "1" | "on" | "yes" => 10f32,
                        _ => BALL_SPEED,
                    }
                } else {
                    BALL_SPEED
                }
            }
        };
        self.rect.w = ball_size().x;
        self.rect.h = ball_size().y;
        self.rect.x += self.velocity.x * dt * speed;
        self.rect.y += self.velocity.y * dt * speed;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
            self.velocity.x = -self.velocity.x;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
            self.velocity.x = -self.velocity.x;
        }
        if self.rect.y < 0f32 {
            self.rect.y = 0f32;
            self.velocity.y = -self.velocity.y;
        }

        self.draw();
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            Color {
                r: 0.2,
                g: 0.0,
                b: 0.2,
                a: 1.0,
            },
        )
    }
}

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub boost: bool,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 3,
            boost: rand::gen_range(0f32, 1f32) < 0.5f32,
        }
    }
    pub fn with_lives(&mut self, lives: i32) {
        self.lives = lives;
    }
    pub fn draw(&self) {
        let color = match self.lives {
            1 => RED,
            2 => Color::new(0.9, 0.9, 0.3, 1.0),
            3 => Color::new(0.0, 0.8, 0.3, 1.0),
            4 => Color::new(0.9, 0.0, 0.3, 1.0),
            5 => Color::new(0.0, 0.0, 0.3, 1.0),
            6 => Color::new(0.9, 0.9, 0.0, 1.0),
            7 => Color::new(0.0, 0.0, 0.0, 1.0),
            8 => Color::new(0.9, 0.0, 0.0, 1.0),
            9 => Color::new(0.0, 0.0, 0.4, 1.0),
            _ => Color::new(0.4, 0.3, 0.3, 1.0),
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
