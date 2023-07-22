use escape::{game::Game, resources::init_resources, *};
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

    let mut frame_count: usize = 0;

    let mut game = Game::new();

    loop {
        let (r, g, b, a) = WINDOW_BACKGROUND_COLOR;
        clear_background(Color::from_rgba(r, g, b, a));
        frame_count += 1;

        game.update(frame_count);
        game.draw(0.0, 0.0);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
