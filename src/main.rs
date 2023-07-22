use escape::{resources::init_resources, *};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Escape".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: IS_FULL_SCREEN,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    init_resources().await;

    loop {
        let (r, g, b, a) = WINDOW_BACKGROUND_COLOR;
        clear_background(Color::from_rgba(r, g, b, a));

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
