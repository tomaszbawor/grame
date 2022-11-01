use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([90f32, 40f32]);

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
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }
}

#[macroquad::main("grame")]
async fn main() {
    let mut player = Player::new();
    let mut blocks = Vec::new();

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

    loop {
        player.update(get_frame_time());
        clear_background(BLACK);

        for block in &blocks {
            block.draw();
        }

        player.draw();

        next_frame().await
    }
}
