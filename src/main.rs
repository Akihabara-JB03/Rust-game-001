use rand::Rng;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
#[macroquad::main("Pong")]
async fn main() {
    let mut x = 10.0;
    let mut y = 240.0;
    let paddle_wid = 20.0;
    let paddle_hei = 60.0;
    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            y -= 5;
        }
        if is_key_down(KeyCode::S) {
            y += 5;
        }
        if y <= 0 {
            y = 0;
        }
        if y >= (600 - paddle_hei) {
            y = (600 - paddle_hei);
        }
        draw_rectangle(x,y,paddle_wid,paddle_hei,WHITE);
        next_frame().await;
    }
}
