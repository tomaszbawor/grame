use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([90f32, 40f32]);
const BALL_SIZE: f32 = 20f32;
const BALL_SPEED: f32 = 500f32;

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0f32;

        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
        }

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        } else if self.rect.x > screen_width() - PLAYER_SIZE.x {
            self.rect.x = screen_width() - PLAYER_SIZE.x;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

struct Block {
    rect: Rect,
    lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
        }
    }

    pub fn draw(&self) {
        let color = match self.lives {
            2 => BLUE,
            1 => RED,
            _ => WHITE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), -1f32 /* always up */).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x > screen_width() - BALL_SIZE {
            self.vel.x = -1f32;
        }

        // Top hit
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    if let Some(intersection) = a.intersect(*b) {
        let collision_vector = b.center() - a.center();
        let collision_signum = collision_vector.signum();

        match intersection.w > intersection.h {
            true => {
                // bounce on y
                a.y -= collision_signum.y * intersection.h;
                match collision_signum.y > 0f32 {
                    true => vel.y = -vel.y.abs(),
                    false => vel.y = vel.y.abs(),
                }
            }
            false => {
                // bounce on x
                a.x -= collision_signum.x * intersection.w;
                match collision_signum.x > 0f32 {
                    true => vel.x = vel.x.abs(),
                    false => vel.x = -vel.x.abs(),
                }
            }
        }
        return true;
    }
    false
}

#[macroquad::main("grame")]
async fn main() {
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    let (horizontal_block_count, vertical_block_count) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * horizontal_block_count as f32)) / 2.0,
        50f32,
    );

    for i in 0..horizontal_block_count * vertical_block_count {
        let block_x = (i % horizontal_block_count) as f32 * total_block_size.x;
        let block_y = (i / horizontal_block_count) as f32 * total_block_size.y;

        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32,
        screen_height() * 0.5f32,
    ))); // Center the ball

    loop {
        // DEBUG PART
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(vec2(
                screen_width() * 0.5f32,
                screen_height() * 0.5f32,
            ))); // Center the ball
        }

        player.update(get_frame_time());

        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }

        // Collision detection
        for ball in balls.iter_mut() {
            resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
            for block in blocks.iter_mut() {
                if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                    block.lives -= 1;
                }
            }
        }

        // remove killed blocks
        blocks.retain(|block| block.lives > 0);

        clear_background(BLACK);

        for block in &blocks {
            block.draw();
        }

        player.draw();
        for ball in &balls {
            ball.draw()
        }

        next_frame().await
    }
}
