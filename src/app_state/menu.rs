use macroquad::prelude::*;
use crate::app_state::AppState;
use crate::engine::input::Input;

pub struct MenuState {
    selected_option: usize,
    options: Vec<&'static str>,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec!["Start Game", "Instructions", "Quit"],
        }
    }
    
    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) && self.selected_option > 0 {
            self.selected_option -= 1;
        }
        
        if is_key_pressed(KeyCode::Down) && self.selected_option < self.options.len() - 1 {
            self.selected_option += 1;
        }
    }
    
    pub fn update(&mut self, _dt: f32) {
        // Menu doesn't need updates
    }
    
    pub fn draw(&self) {
        clear_background(BLACK);
        
        // Draw title
        let title = "TANKS";
        let title_size = 60.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            400.0 - title_dims.width / 2.0,
            150.0,
            title_size,
            YELLOW,
        );
        
        // Draw menu options
        let start_y = 300.0;
        for (i, option) in self.options.iter().enumerate() {
            let color = if i == self.selected_option { YELLOW } else { WHITE };
            let y = start_y + (i as f32 * 50.0);
            let dims = measure_text(option, None, 30, 1.0);
            draw_text(
                option,
                400.0 - dims.width / 2.0,
                y,
                30.0,
                color,
            );
            
            // Draw selection arrow
            if i == self.selected_option {
                draw_text(">", 400.0 - dims.width / 2.0 - 30.0, y, 30.0, YELLOW);
            }
        }
        
        // Draw controls
        draw_text(
            "Controls: WASD/Arrows - Move, Space - Fire, ESC - Pause",
            50.0,
            550.0,
            16.0,
            GRAY,
        );
    }
    
    pub fn next_state(self) -> AppState {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match self.selected_option {
                0 => AppState::Game(super::GameState::new()),
                1 => AppState::Menu(self), // Instructions would be another state
                2 => AppState::Quit,
                _ => AppState::Menu(self),
            }
        } else {
            AppState::Menu(self)
        }
    }
}
