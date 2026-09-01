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
    let mut y = 200.0;
    let mut x_2 = 790.0;
    let mut y_2 = 200.0;
    let paddle_wid = 20.0;
    let paddle_hei = 100.0;
    let ball_radius = 5.0;
    let mut ball_x = 400.0 - (ball_radius * 2.0);
    let mut ball_y = 300.0 - (ball_radius * 2.0);
    let mut ball_speed_x = 5.0;
    let mut ball_speed_y = 5.0;
    
    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            y -= 5.0;
        }
        if is_key_down(KeyCode::S) {
            y += 5.0;
        }
        if is_key_down(KeyCode::Up) {
            y_2 -= 5.0;
        }
        if is_key_down(KeyCode::Down) {
            y_2 += 5.0;
        }
        if y <= 0.0 {
            y = 0.0;
        }
        if y >= 600.0 - paddle_hei {
            y = 600.0 - paddle_hei;
        }
        if y_2 <= 0.0 {
            y_2 = 0.0;
        }
        if y_2 >= 600.0 - paddle_hei {
            y_2 = 600.0 - paddle_hei;
        }
        if (ball_y <= 0.0 || ball_y >= 600.0 - ball_radius) {
            ball_speed_y = -ball_speed_y;
        }
        if ball_y <= y+paddle_hei && ball_y >= y {
            if ball_x <= x + 5.0 {
                ball_speed_x = ball_speed_x;
            }
        }
        if ball_y <= y_2+paddle_hei && ball_y >= y_2 {
            if ball_x <= x_2 - 5.0 {
                ball_speed_x = -ball_speed_x;
            }
        }
        draw_rectangle(x,y,paddle_wid,paddle_hei,WHITE);
        draw_rectangle(x_2,y_2,paddle_wid,paddle_hei,WHITE);
        ball_x += ball_speed_x;
        ball_y += ball_speed_y;
        draw_circle(ball_x, ball_y, ball_radius, WHITE);
        next_frame().await;
    }
}
