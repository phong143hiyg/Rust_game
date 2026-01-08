use macroquad::prelude::*;
use crate::engine::types::Direction;

pub struct Input;

impl Input {
    #[allow(dead_code)]
    pub fn is_key_down(key: KeyCode) -> bool {
        is_key_down(key)
    }

    #[allow(dead_code)]
    pub fn is_key_pressed(key: KeyCode) -> bool {
        is_key_pressed(key)
    }
    
    pub fn get_player1_movement_direction() -> Option<Direction> {
        if is_key_down(KeyCode::W)  {
            Some(Direction::Up)
        } else if is_key_down(KeyCode::S) {
            Some(Direction::Down)
        } else if is_key_down(KeyCode::A) {
            Some(Direction::Left)
        } else if is_key_down(KeyCode::D) {
            Some(Direction::Right)
        } else {
            None
        }
    }
    
    pub fn is_player1_fire_pressed() -> bool {
        is_key_pressed(KeyCode::Space)
    }
    
    // Player 2 controls (IJKL for movement)
    pub fn get_player2_movement_direction() -> Option<Direction> {
        if is_key_down(KeyCode::Up) {
            Some(Direction::Up)
        } else if is_key_down(KeyCode::Down) {
            Some(Direction::Down)
        } else if is_key_down(KeyCode::Left) {
            Some(Direction::Left)
        } else if is_key_down(KeyCode::Right) {
            Some(Direction::Right)
        } else {
            None
        }
    }
    
    pub fn is_player2_fire_pressed() -> bool {
        is_key_pressed(KeyCode::Enter)
    }

    #[allow(dead_code)]
    pub fn is_escape_pressed() -> bool {
        is_key_pressed(KeyCode::Escape)
    }
}
