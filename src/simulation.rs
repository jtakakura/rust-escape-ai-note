use macroquad::prelude::*;

use crate::{game::Game, NUM_FRAMES};

pub struct Simulation {
    game: Game,
    frame_count: usize,
    generation_count: u32,
}

pub struct SimulationStats {
    pub frame_count: usize,
    pub generation_count: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            frame_count: 0,
            generation_count: 1,
        }
    }

    pub fn update(&mut self) -> Option<SimulationStats> {
        self.game.update(self.frame_count);
        self.frame_count += 1;

        if self.frame_count >= NUM_FRAMES {
            self.start_new_generation();
        }

        Some(SimulationStats {
            frame_count: self.frame_count,
            generation_count: self.generation_count,
        })
    }

    fn start_new_generation(&mut self) {
        self.game = Game::new();

        self.frame_count = 0;
        self.generation_count += 1;
    }

    pub fn draw(&self) {
        self.game.draw(0.0, 0.0);
    }
}

impl SimulationStats {
    pub fn new() -> Self {
        Self {
            frame_count: 1,
            generation_count: 1,
        }
    }
}
