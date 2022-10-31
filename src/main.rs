use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);


struct Player {
    rect:  Rect,
}


impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn update(&mut self) {
        self.rect.x = mouse_position().0 - PLAYER_SIZE.x * 0.5f32;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        } else if self.rect.x > screen_width() - PLAYER_SIZE.x {
            self.rect.x = screen_width() - PLAYER_SIZE.x;
        }
    }
}

#[macroquad::main("grame")]
async fn main() {

    let mut player = Player::new();

        loop {
            player.update();
            clear_background(BLACK);

            draw_rectangle(
                player.rect.x,
                player.rect.y,
                player.rect.w,
                player.rect.h,
                WHITE
            );

            next_frame().await
        }

}
