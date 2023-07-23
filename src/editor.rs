use macroquad::prelude::*;

use crate::simulation::SimulationStats;

pub struct Settings {
    pub is_pause: bool,
    pub is_draw: bool,
    pub is_restart: bool,
    pub is_frame_skip: bool,
    pub is_random_ai: bool,
    pub is_show_egui: bool,
    pub is_ai_enabled: bool,
    pub is_show_multiple: bool,
    pub slow_mode: bool,
}

pub struct Editor {
    pub settings: Settings,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            is_pause: false,
            is_draw: true,
            is_restart: false,
            is_frame_skip: false,
            is_ai_enabled: true,
            is_show_egui: false,
            is_show_multiple: false,
            is_random_ai: false,
            slow_mode: false,
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    pub fn update(&mut self) {
        // Handle keyboard input
        if is_key_pressed(KeyCode::Space) {
            self.settings.is_pause = !self.settings.is_pause;
        }
        if is_key_pressed(KeyCode::Tab) {
            self.settings.is_show_egui = !self.settings.is_show_egui;
        }
        if is_key_pressed(KeyCode::R) {
            self.settings.is_restart = true;
        }
        if is_key_pressed(KeyCode::Backspace) {
            self.settings.slow_mode = !self.settings.slow_mode;
        }
        if is_key_pressed(KeyCode::Backslash) {
            self.settings.is_ai_enabled = !self.settings.is_ai_enabled;
        }
        if is_key_pressed(KeyCode::RightShift) {
            self.settings.is_frame_skip = !self.settings.is_frame_skip;
        }
    }

    pub fn draw(&mut self, _stats: &SimulationStats) {
        if !self.settings.is_show_egui {
            return;
        }
    }
}
