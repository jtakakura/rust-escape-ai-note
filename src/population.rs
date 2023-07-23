use macroquad::prelude::*;

use crate::{
    agent::AgentCommand, editor::Editor, game::Game, FRAME_SCALE, NUM_GAMES, NUM_GAMES_IN_ROW,
    UNIT_FRAME_SIZE,
};

pub struct Population {
    games: Vec<Game>,
}

impl Population {
    pub fn new() -> Self {
        Self {
            games: (0..NUM_GAMES).map(|_| Game::new()).collect(),
        }
    }

    pub fn update(&mut self, frame_count: usize, editor: &Editor) {
        // User input applies only to 1st game
        self.handle_user_input();

        if !editor.settings.is_ai_enabled {
            return;
        }

        self.games.iter_mut().for_each(|g| g.update(frame_count))
    }

    fn handle_user_input(&mut self) {
        if is_key_pressed(KeyCode::W) {
            self.games[0].update_manual(AgentCommand::Top);
        } else if is_key_pressed(KeyCode::A) {
            self.games[0].update_manual(AgentCommand::Left);
        } else if is_key_pressed(KeyCode::S) {
            self.games[0].update_manual(AgentCommand::Bottom);
        } else if is_key_pressed(KeyCode::D) {
            self.games[0].update_manual(AgentCommand::Right);
        }
    }

    pub fn draw(&self, editor: &Editor) {
        if !editor.settings.is_show_multiple {
            self.games[0].draw(0.0, 0.0);
            return;
        }

        // Draw all games
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;
        let grid_padding = 40.0;

        let (w, h) = self.games[0].lvl.size;
        let (w, h) = (w as f32, h as f32);
        let (w, h) = (
            w * UNIT_FRAME_SIZE * FRAME_SCALE,
            h * UNIT_FRAME_SIZE * FRAME_SCALE,
        );

        for g in self.games.iter() {
            g.draw(offset_x, offset_y);

            offset_x += w * grid_padding;

            if offset_x >= w + NUM_GAMES_IN_ROW as f32 {
                offset_y += h + grid_padding;
                offset_x = 0.0;
            }
        }
    }
}
