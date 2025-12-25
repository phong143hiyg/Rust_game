use macroquad::prelude::*;
use crate::engine::types::Direction;

pub struct Input;

impl Input {
    pub fn is_key_down(key: KeyCode) -> bool {
        is_key_down(key)
    }
    
    pub fn is_key_pressed(key: KeyCode) -> bool {
        is_key_pressed(key)
    }
    
    pub fn get_movement_direction() -> Option<Direction> {
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            Some(Direction::Up)
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            Some(Direction::Down)
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            Some(Direction::Left)
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            Some(Direction::Right)
        } else {
            None
        }
    }
    
    pub fn is_fire_pressed() -> bool {
        is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter)
    }
    
    pub fn is_escape_pressed() -> bool {
        is_key_pressed(KeyCode::Escape)
    }
}
